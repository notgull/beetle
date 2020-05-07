/* -----------------------------------------------------------------------------------
 * src/object/linux/x11/main_window.rs - X11 main window implementation.
 * beetle - Simple graphics framework for Rust
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

use super::{
    do_upgrade,
    gui_object::{GuiObject, MainWindowBase, WindowBase},
    DisplayPointer, X11Display,
};
use nalgebra::Point4;
use std::{
    fmt,
    marker::PhantomData,
    os::raw::c_int,
    ptr::{self, NonNull},
};
use x11::xlib::{self, Display, Window, GC};

/// Flags that can be passed to the X11Window through generics.
pub trait X11WindowType: Sized + fmt::Debug {
    /// The child to pass in.
    type ExpectedChild;

    /// Whether or not we are the main window.
    fn is_main() -> bool;
    /// Determine what this window's parent will be.
    fn get_parent(parent: Self::ExpectedChild) -> Option<Window>;
}

/// Indicates that this X11Window is the main window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X11MainWindow;

impl X11WindowType for X11MainWindow {
    type ExpectedChild = ();

    #[inline]
    fn is_main() -> bool {
        true
    }

    #[inline]
    fn get_parent(_parent: ()) -> Option<Window> {
        None
    }
}

/// Indicates that this X11Window is a child window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X11ChildWindow;

impl X11WindowType for X11ChildWindow {
    type ExpectedChild = Window;

    #[inline]
    fn is_main() -> bool {
        false
    }

    #[inline]
    fn get_parent(parent: Window) -> Option<Window> {
        Some(parent)
    }
}

#[derive(Debug)]
pub struct X11Window<WindowType: X11WindowType> {
    display: DisplayPointer,
    window: Window,
    gc: GC,

    bounds: Point4<u32>,
    _phantom: PhantomData<WindowType>,
}

unsafe impl<WT: X11WindowType> Send for X11Window<WT> { }
unsafe impl<WT: X11WindowType> Sync for X11Window<WT> { }

impl<WindowType: X11WindowType> X11Window<WindowType> {
    pub fn new(
        display: &X11Display,
        parent: WindowType::ExpectedChild,
        bounds: Point4<u32>,
        title: &str,
    ) -> Result<Self, crate::Error> {
        let weak_ptr = display.get_display_ref();
        let dpy = do_upgrade(&weak_ptr)?.as_ref().clone().as_ptr();
        let window = unsafe {
            xlib::XCreateSimpleWindow(
                dpy,
                WindowType::get_parent(parent)
                    .or_else(|| Some(xlib::XRootWindow(dpy, display.screen())))
                    .unwrap(),
                bounds.x as c_int,
                bounds.y as c_int,
                bounds.z,
                bounds.w,
                1,
                display.white_pixel(),
                display.black_pixel(),
            )
        };

        // set input method
        unsafe {
            xlib::XSelectInput(
                dpy,
                window,
                xlib::ExposureMask | xlib::ButtonPressMask | xlib::KeyPressMask,
            )
        };

        // also set the title
        unsafe { xlib::XStoreName(dpy, window, crate::utils::to_cstring(title)?) };

        // create a GC to draw with
        let gc = unsafe { xlib::XCreateGC(dpy, window, 0, ptr::null_mut()) };

        Ok(Self {
            display: weak_ptr,
            window,
            gc,
            bounds,
            _phantom: PhantomData,
        })
    }
}

impl<T: X11WindowType> WindowBase for X11Window<T> {
    fn set_title(&mut self, val: &str) -> Result<(), crate::Error> {
        unsafe {
            xlib::XStoreName(
                do_upgrade(&self.display)?.as_ptr(),
                self.window,
                crate::utils::to_cstring(val)?,
            )
        };
        Ok(())
    }
}

impl MainWindowBase for X11Window<X11MainWindow> {}

impl<WindowType: X11WindowType> GuiObject for X11Window<WindowType> {
    #[inline]
    fn bounds(&self) -> Point4<u32> {
        self.bounds
    }
    fn set_bounds(&mut self, bounds: Point4<u32>) -> Result<(), crate::Error> {
        self.bounds = bounds;
        unsafe {
            xlib::XMoveResizeWindow(
                do_upgrade(&self.display)?.as_ptr(),
                self.window,
                bounds.x as i32,
                bounds.y as i32,
                bounds.z,
                bounds.w,
            )
        };
        Ok(())
    }
    fn set_parent(&mut self, parent: &dyn GuiObject) -> Result<(), crate::Error> {
        if WindowType::is_main() {
            // TODO: there's probably a way to make this a compile error
            Err(crate::Error::RootWindowCannotReassignParent)
        } else {
            match parent.get_x11_window() {
                Some(p) => {
                    unsafe {
                        xlib::XReparentWindow(
                            do_upgrade(&self.display)?.as_ptr(),
                            self.window,
                            p,
                            0,
                            0,
                        )
                    };
                    Ok(())
                }
                None => Err(crate::Error::NoSubelementParent),
            }
        }
    }
    #[inline]
    fn get_x11_window(&self) -> Option<Window> {
        Some(self.window)
    }
    #[inline]
    fn render(
        &self,
        _display: &NonNull<Display>,
        _win: Window,
        _gc: GC,
    ) -> Result<(), crate::Error> {
        Ok(())
    }
}
