/* -----------------------------------------------------------------------------------
 * src/object/linux/x11/mod.rs - This should define a handful of objects that
 *                               implement the basic GuiObject traits.
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

pub(crate) use super::super::{
    gui_object::{self, GuiObject, WindowBase},
    GuiFactoryBase,
};
use crate::{Font, GenericWidget, MainWindow, Widget};
use nalgebra::geometry::Point4;
use std::{
    convert::AsMut,
    mem,
    os::raw::{c_int, c_ulong},
    ptr::{self, NonNull},
    sync::{Arc, Weak},
};
use x11::xlib::{self, Atom, Display, Window, _XGC};

mod label;
pub use label::*;
mod window;
pub use window::*;

/// The X11 Display. A reference to this should be carried in every X11 window.
#[derive(Debug)]
pub struct X11Display {
    display: Arc<NonNull<Display>>,
    screen: c_int,
    black_pixel: c_ulong,
    white_pixel: c_ulong,
}

impl X11Display {
    #[inline]
    pub fn get_display_ref(&self) -> Weak<NonNull<Display>> {
        Arc::downgrade(&self.display)
    }

    #[inline]
    pub fn screen(&self) -> c_int {
        self.screen
    }

    #[inline]
    pub fn black_pixel(&self) -> c_ulong {
        self.black_pixel
    }

    #[inline]
    pub fn white_pixel(&self) -> c_ulong {
        self.white_pixel
    }
}

impl GuiFactoryBase for X11Display {
    type ChildWindow = X11Window<X11ChildWindow>;
    type MainWindow = X11Window<X11MainWindow>;
    type Label = X11Label;

    fn new() -> Result<Self, crate::Error> {
        let mut display = NonNull::new(unsafe { xlib::XOpenDisplay(ptr::null()) })
            .ok_or_else(|| crate::Error::UnableToOpenDisplay)?;
        let screen = unsafe { xlib::XDefaultScreen(display.as_mut()) };
        let black_pixel = unsafe { xlib::XBlackPixel(display.as_mut(), screen) };
        let white_pixel = unsafe { xlib::XWhitePixel(display.as_mut(), screen) };
        Ok(Self {
            display: Arc::new(display),
            screen,
            black_pixel,
            white_pixel,
        })
    }

    fn main_window(
        &self,
        bounds: Point4<u32>,
        title: &str,
    ) -> Result<Self::MainWindow, crate::Error> {
        X11Window::<X11MainWindow>::new(self, (), bounds, title)
    }

    fn label<T: GuiObject>(
        &self,
        _parent: &T,
        bounds: Point4<u32>,
        text: &str,
        font: Option<&Font>,
    ) -> Result<Self::Label, crate::Error> {
        Ok(X11Label::new(bounds, text, font))
    }

    fn child_window<T: WindowBase>(
        &self,
        parent: &T,
        bounds: Point4<u32>,
        title: &str,
    ) -> Result<Self::ChildWindow, crate::Error> {
        X11Window::<X11ChildWindow>::new(self, parent.get_x11_window().unwrap(), bounds, title)
    }

    fn main_loop(self, window: Widget<MainWindow>) -> Result<(), crate::Error> {
        // set up x11 event loop
        let mut xevent: xlib::XEvent = unsafe { mem::zeroed() };
        let d_ptr = self.display.as_ptr();
        'main: loop {
            unsafe { xlib::XNextEvent(d_ptr, &mut xevent) };

            if unsafe { xevent.type_ } == xlib::Expose {
                // draw all subcomponents
                let inner_lock = window.internal().try_borrow()?;
                let inner = inner_lock.inner();
                render_tree(
                    &self.display,
                    &window,
                    inner.get_x11_window().unwrap(),
                    inner.get_x11_gc().unwrap(),
                )?;
            }

            if (unsafe { xevent.type_ } == xlib::ClientMessage)
                && (AsMut::<[Atom]>::as_mut(&mut unsafe { xevent.client_message.data })[0]
                    == window.internal().try_borrow()?.inner().delete_window_atom())
            {
                break 'main;
            }
        }
        Ok(())
    }
}

// render components down the render tree
fn render_tree(
    display: &NonNull<Display>,
    widget: &dyn GenericWidget,
    current_window: Window,
    gc: NonNull<_XGC>,
) -> Result<(), crate::Error> {
    // render the current component
    let inner = widget.inner_generic()?;
    inner.render(display, current_window, gc)?;

    // iterate over children
    widget
        .children()?
        .into_iter()
        .map(|w| match w.inner_generic()?.get_x11_window() {
            Some(next_window) => {
                let next_gc = inner.get_x11_gc().unwrap();
                render_tree(display, w, next_window, next_gc)
            }
            None => render_tree(display, w, current_window, gc),
        })
        .collect::<Result<Vec<()>, crate::Error>>()?;

    Ok(())
}

impl Drop for X11Display {
    fn drop(&mut self) {
        unsafe { xlib::XCloseDisplay(self.display.as_ref().clone().as_mut()) };
    }
}

/// Type alias for a weak pointer to the current display.
pub type DisplayPointer = Weak<NonNull<Display>>;

pub(crate) fn do_upgrade(ptr: &DisplayPointer) -> Result<Arc<NonNull<Display>>, crate::Error> {
    ptr.upgrade().ok_or_else(|| crate::Error::DroppedDisplay)
}
