/* -----------------------------------------------------------------------------------
 * src/object/linux/x11/mod.rs - This file should define the X11 GUI manager for
 *                               Linux. This module handles the creation of X11
 *                               objects and calling methods on them.
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

use super::super::{ApplicationObject, GuiObject};
use crate::utils::to_cstring;
use std::{
    boxed::Box,
    cell::RefCell,
    fmt,
    os::raw::{c_int, c_ulong},
    ptr,
    rc::Rc,
};
use x11::xlib::{self, Display, Window, GC};

#[derive(Debug)]
pub struct LApplicationObject {
    display: *mut Display,
    black_pixel: c_ulong,
    white_pixel: c_ulong,
    s: c_int,
}

impl LApplicationObject {
    #[inline]
    pub fn new(display: *mut Display) -> Self {
        let s = unsafe { xlib::XDefaultScreen(display) };
        let black_pixel = unsafe { xlib::XBlackPixel(display, s) };
        let white_pixel = unsafe { xlib::XWhitePixel(display, s) };
        Self {
            display,
            black_pixel,
            white_pixel,
            s,
        }
    }

    #[inline]
    pub fn display(&self) -> *mut Display {
        self.display
    }

    #[inline]
    pub fn black_pixel(&self) -> c_ulong {
        self.black_pixel
    }

    #[inline]
    pub fn white_pixel(&self) -> c_ulong {
        self.white_pixel
    }

    #[inline]
    pub fn screen(&self) -> c_int {
        self.s
    }
}

impl ApplicationObject for LApplicationObject {}

impl Drop for LApplicationObject {
    fn drop(&mut self) {
        unsafe { xlib::XCloseDisplay(self.display) };
    }
}

/// Represents a sub-object, or an element not expressed as an X11 window.
pub trait SubObject: fmt::Debug {
    fn render(&self) -> Result<(), crate::Error>;
}

#[derive(Debug)]
pub enum LGuiObject {
    Window {
        window: Window,
        gc: GC,
        app: Rc<LApplicationObject>,
    },
    SubObject(Box<dyn SubObject>),
}

// general function for window creation
fn create_x_window(
    app: &Rc<LApplicationObject>,
    parent: Window,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    title: &str,
) -> Result<LGuiObject, crate::Error> {
    let display = app.display();

    // create window and gc
    let window = unsafe {
        xlib::XCreateSimpleWindow(
            display,
            parent,
            x as c_int,
            y as c_int,
            w,
            h,
            1,
            app.white_pixel(),
            app.black_pixel(),
        )
    };
    let gc = unsafe { xlib::XCreateGC(display, window, 0, ptr::null_mut()) };

    // set the window title
    unsafe { xlib::XStoreName(display, window, to_cstring(title)?) };

    unsafe {
        xlib::XSelectInput(display, window, xlib::ExposureMask | xlib::KeyPressMask);
        xlib::XMapWindow(display, window);
    }

    Ok(LGuiObject::Window {
        window,
        gc,
        app: app.clone(),
    })
}

impl GuiObject for LGuiObject {
    type AObject = Rc<LApplicationObject>;

    // create an application
    fn application() -> Result<Self::AObject, crate::Error> {
        let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
        if display.is_null() {
            Err(crate::Error::UnableToOpenDisplay)
        } else {
            Ok(Rc::new(LApplicationObject::new(display)))
        }
    }

    // create the main window for the application
    fn main_window(
        app: &Self::AObject,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        title: &str,
    ) -> Result<Self, crate::Error> {
        create_x_window(
            app,
            unsafe { xlib::XDefaultRootWindow(app.display()) },
            x,
            y,
            w,
            h,
            title,
        )
    }

    // create a child window
    fn child_window(
        app: &Self::AObject,
        parent: &Self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        title: &str,
    ) -> Result<Self, crate::Error> {
        match *parent {
            LGuiObject::Window { window, .. } => create_x_window(app, window, x, y, w, h, title),
            _ => Err(crate::Error::ExpectedWindow),
        }
    }

    // create a checkbox
    fn checkbox(
        app: &Self::AObject,
        parent: &Self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        text: &str,
    ) -> Result<Self, crate::Error> {
        unimplemented!()
    }

    // create a label
    fn label(
        app: &Self::AObject,
        parent: &Self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        text: &str,
    ) -> Result<Self, crate::Error> {
        unimplemented!()
    }

    // set bounds
    fn set_rect(x: u32, y: u32, w: u32, h: u32) -> Result<(), crate::Error> {
        unimplemented!()
    }
}

impl Drop for LGuiObject {
    fn drop(&mut self) {
        if let LGuiObject::Window { window, gc, app } = self {
            let display = app.display();
            unsafe {
                xlib::XFreeGC(display, *gc);
                xlib::XDestroyWindow(display, *window);
            }
        }
    }
}
