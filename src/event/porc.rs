/* -----------------------------------------------------------------------------------
 * src/event/porc.rs - Translate a Win32 MSG to an event.
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

use super::{Event, EventType};
use crate::{Instance, KeyInfo, KeyType, Window};
use euclid::default::{Point2D, Rect, Size2D};
use porcupine::{
    prelude::*,
    winapi::{
        shared::{
            minwindef::{DWORD, HIWORD, LOWORD, LPARAM, UINT, WPARAM},
            windef::{HWND, LPRECT},
        },
        um::winuser::*,
    },
};
use smallvec::SmallVec;
use std::{
    boxed::Box,
    convert::TryInto,
    mem,
    os::raw::c_int,
    ptr,
    sync::{atomic::AtomicPtr, Arc},
};

const OLD_BOUNDS_NOT_FOUND: &'static str = "Old bounds were not stored in the window object.";

impl Event {
    pub(crate) fn from_porc(
        instance: &Instance,
        assoc_window: Window,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> crate::Result<SmallVec<[Event; 2]>> {
        let mut evs: SmallVec<[Event; 2]> = SmallVec::new();

        log::debug!("Received windows message: {}", msg);

        #[inline]
        fn get_newbounds(lp: LPARAM) -> crate::Result<Rect<u32>> {
            let new_winpos: LPWINDOWPOS = unsafe { mem::transmute::<LPARAM, LPWINDOWPOS>(lp) };
            let new_wp: WINDOWPOS = unsafe { ptr::read(new_winpos) };

            let new_bounds: Rect<u32> = euclid::rect(
                new_wp.x.try_into()?,
                new_wp.y.try_into()?,
                new_wp.cx.try_into()?,
                new_wp.cy.try_into()?,
            );

            Ok(new_bounds)
        }

        match msg {
            WM_CLOSE => {
                log::debug!("Found WM_CLOSE message");
                evs.push(Event::new(&assoc_window, EventType::Close, vec![]));
            }
            WM_PAINT => {
                log::debug!("Found WM_PAINT message");
                evs.push(Event::new(&assoc_window, EventType::Paint, vec![]));
            }
            // for all intents and purposes these are the same thing
            WM_KEYUP | WM_SYSKEYUP | WM_KEYDOWN | WM_SYSKEYDOWN => {
                log::debug!("Found keyboard-related message");
                let key_stroke = wparam;
                let mut ki = KeyInfo::new(KeyType::from_vk(key_stroke));

                // set key information
                fn set_key_state<F>(ki: &mut KeyInfo, test_for: c_int, setter: F)
                where
                    F: FnOnce(&mut KeyInfo),
                {
                    if unsafe { GetKeyState(test_for) } & std::i16::MAX != 0 {
                        setter(ki);
                    }
                }

                set_key_state(&mut ki, VK_CONTROL, |ki| ki.set_ctrl(true));
                set_key_state(&mut ki, VK_MENU, |ki| ki.set_alt(true));
                set_key_state(&mut ki, VK_SHIFT, |ki| ki.set_shift(true));

                let loc: Option<Point2D<u32>> = match porcupine::cursor_pos()
                    .and_then(|f| assoc_window.inner_porc_window().screen_to_client(f))
                {
                    Err(e) => {
                        // if an error occurred, just drop it and set loc to None
                        log::error!("Error finding position on screen: {}", e);
                        None
                    }
                    Ok(p) => {
                        if let (Ok(x), Ok(y)) = (p.x.try_into(), p.y.try_into()) {
                            Some(Point2D::new(x, y))
                        } else {
                            None
                        }
                    }
                };

                evs.push(Event::new(
                    &assoc_window,
                    match msg {
                        WM_KEYUP | WM_SYSKEYUP => EventType::KeyUp,
                        WM_KEYDOWN | WM_SYSKEYDOWN => EventType::KeyDown,
                        _ => unreachable!(),
                    },
                    vec![Arc::new(ki), Arc::new(loc)],
                ));
            }
            WM_WINDOWPOSCHANGING => {
                let new_bounds = get_newbounds(lparam)?;
                let old_bounds = assoc_window.bounds();

                // make sure to store the bounds
                assoc_window.store_old_bounds();

                // the (false, false) asserts that it's from the event loop and that a BoundsChanged event
                // should not be emitted
                evs.push(Event::new(
                    &assoc_window,
                    EventType::BoundsChanging,
                    vec![
                        Arc::new(old_bounds),
                        Arc::new(new_bounds),
                        Arc::new((false, false)),
                    ],
                ));
            }
            WM_WINDOWPOSCHANGED => {
                log::debug!("Found WM_WINDOWPOSCHANGED");
                let new_bounds: Rect<u32> = get_newbounds(lparam)?;
                let old_bounds = match assoc_window.take_old_bounds() {
                    Some(ob) => ob,
                    None => {
                        log::error!("{}", OLD_BOUNDS_NOT_FOUND);
                        // just re-use the new bounds
                        new_bounds
                    }
                };

                evs.push(Event::new(
                    &assoc_window,
                    EventType::BoundsChanged,
                    vec![Arc::new(old_bounds), Arc::new(new_bounds)],
                ));
            }
            _ => {
                log::debug!("Unsupported message.");
            }
        }

        // also push an event that "carries" the message
        evs.push(Event::new(
            &assoc_window,
            EventType::MessageCarrier,
            vec![Arc::new(msg), Arc::new(wparam), Arc::new(lparam)],
        ));

        log::debug!("Queueing events: {:?}", &evs);

        Ok(evs)
    }
}
