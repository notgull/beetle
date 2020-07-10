/* -----------------------------------------------------------------------------------
 * src/wndproc.rs - The window procedure used for Beetle widgets.
 * beetle - Pull-based GUI framework.
 * Copyright © 2020 not_a_seagull
 *
 * This project is licensed under either the Apache 2.0 license or the MIT license, at
 * your option. For more information, please consult the LICENSE-APACHE or LICENSE-MIT
 * files in the repository root.
 * -----------------------------------------------------------------------------------
 * MIT License:
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the “Software”), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 * -----------------------------------------------------------------------------------
 * Apache 2.0 License Declaration:
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ----------------------------------------------------------------------------------
 */

#![cfg(windows)]

use crate::{Event, Instance, Window};
use core::{any::Any, mem};
use maybe_uninit::MaybeUninit;
use porcupine::winapi::{
    shared::{
        basetsd::LONG_PTR,
        minwindef::{FALSE, LPARAM, LRESULT, TRUE, UINT, WPARAM},
        windef::{HBRUSH, HWND},
    },
    um::{errhandlingapi, winuser::*},
};

/// The window procedure used by Beetle.
///
/// TODO: a lot of this could be incorporated into porcupine
pub unsafe extern "system" fn beetle_wndproc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    /*
     *  I'm going to try my best to explain the system I have here, since it's a little bit
     *  convoluted, and it feels like this is the best place to explain it.
     *
     *  The previous iteration of this system would intercept events from the GetMessage
     *  function and then translate those into Beetle events. After the Beetle events
     *  were handled, it would dispatch them to the actual window. However, for some reason,
     *  it seems that GetMessage doesn't actually intercept every message that gets sent
     *  to the windows. Only the window proc can actually receive them.
     *
     *  Here is the system I'm trying here:
     *
     * 1). When the Window object is created, the GWLP_USERDATA pointer within the HWND
     *     is set to a leaked instance of a clone of the Window. The Window clone also
     *     contains a reference to the current Instance, which ensures we have both
     *     during the Window Procedure.
     * 2). When the Instance::next_event() function is called, it will call the usual
     *     Win32 event loop (e.g. GetMessage, TranslateMessage, DispatchMessage). The
     *     DispatchMessage function should call this window procedure. Note that all of
     *     these operations should be synchronous, so we shouldn't have to worry about
     *     any data races. I've still locked the next_events variable in a Mutex to
     *     keep the Instance thread-safe.
     * 3). Inside of this function, given the message information, we will retrieve the
     *     Window pointer (and thus, the instance pointer) from GWLP_USERDATA. After doing
     *     some win32-esque checks (such as checking for close), we will call the Event::from_porc
     *     function to generate a list of Beetle events.
     * 4). New events will be pushed into the Instance.
     */

    // if this is not a window, return
    if IsWindow(hwnd) == FALSE {
        log::warn!("Ran beetle_wndproc with a handle that isn't a window. Deferring to default window proc.");
        return DefWindowProcA(hwnd, msg, wparam, lparam);
    }

    // set the handle containing instance in WC_NCCREATE
    if msg == WM_NCCREATE {
        let create_struct_ptr = mem::transmute::<LPARAM, LPCREATESTRUCTA>(lparam);
        let window_object_ptr = (*create_struct_ptr).lpCreateParams;

        // if we're in debug mode, check to ensure that we have an actual Window here
        #[cfg(debug_assertions)]
        {
            if window_object_ptr.is_null() {
                log::error!(
                    "Pointer to additional parameter is null. This is likely an internal error."
                );
                return FALSE as LRESULT;
            }
        }

        // set to GWLP_USERDATA
        SetWindowLongPtrA(
            hwnd,
            GWLP_USERDATA,
            window_object_ptr as *const () as LONG_PTR,
        );

        return DefWindowProcA(hwnd, msg, wparam, lparam);
    }

    // get the pointer to the instance
    let instance = GetWindowLongPtrA(hwnd, GWLP_USERDATA);

    // if the pointer is null, just return
    if instance == 0 {
        // if there was an error, delete the error part
        let err = errhandlingapi::GetLastError();
        if err == 1812 {
            // this seems to happen no matter what
            // TODO: prevent this from happening
            log::info!("GetWindowLongPtr threw error 1812. This is expected.");
            errhandlingapi::SetLastError(0);
        } else if err != 0 {
            log::error!(
                "GetWindowLongPtrA threw error {}. This is ignored, deferring to default window procedure.", 
                err
            );
            errhandlingapi::SetLastError(0);
        } else {
            // this is likely an internal error
            log::error!(
                "GetWindowLongPtrA returned a null pointer. This is likely an internal error."
            );
        }

        return DefWindowProcA(hwnd, msg, wparam, lparam);
    }

    // transmute to a reference to the window
    let instance = mem::transmute::<LONG_PTR, *const Instance>(instance);
    let instance: &Instance = &*instance;

    let window = match instance.porcupine_get_window(hwnd) {
        Some(w) => w,
        None => {
            log::error!(
                "Window pointer \"{:p}\" does not correspond to a Beetle window. This occurred on message {}.",
                hwnd,
                msg,
            );
            return DefWindowProcA(hwnd, msg, wparam, lparam);
        }
    };

    log::debug!("Found window with ID {}", window.id());

    // some basic handling, now that we have the window
    match msg {
        WM_CLOSE => {
            DestroyWindow(hwnd);
        }
        WM_DESTROY => {
            if match window.is_top_level() {
                Ok(tl) => tl,
                Err(e) => {
                    log::error!(
                        "Unable to determine whether the window is top level: {}. Assuming that it isn't.", 
                        e
                    );
                    false
                }
            } {
                PostQuitMessage(0);
                return 0;
            }
        }
        _ => (),
    }

    // get the events and set the instance's buffer
    let events = Event::from_porc(instance, window, msg, wparam, lparam);
    instance.porcupine_set_next_events(events); // forward the error to the actual Rust part

    // just forward the event to DefWindowProcA now
    DefWindowProcA(hwnd, msg, wparam, lparam)
}
