/* -----------------------------------------------------------------------------------
 * api/flutterbug/src/window.rs - A window in X11 terms. This struct stores the int
 *                                representing the window, as well as its associated
 *                                items.
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

use super::{FlutterbugError, ColorMap, DisplayReference, GenericDisplay, HasXID};
use std::{mem, ptr::{self, NonNull}};
use x11::xlib::{self, _XGC};

/// An X11 window. This usually represents a rectangle of pixels on the screen.
#[derive(Debug)]
pub struct Window {
    win: xlib::Window,
    dpy: DisplayReference,
    // window should also store a reference to its GC and Colormap
    gc: NonNull<_XGC>,
    colormap: ColorMap,
}

impl Window {
    #[inline]
    pub(crate) fn from_raw(
        win: xlib::Window,
        dpy: DisplayReference,
    ) -> Result<Self, FlutterbugError> {
        // create the graphics context
        let gc = unsafe { xlib::XCreateGC(dpy.raw()?.as_mut(), win, 0, ptr::null_mut()) };
        let gc = NonNull::new(gc).ok_or_else(|| FlutterbugError::UnableToCreateGC)?;

        // get the pointer to the visual item
        let mut xattrs: xlib::XWindowAttributes = unsafe { mem::zeroed() };
        unsafe { xlib::XGetWindowAttributes(dpy.raw()?.as_mut(), win, &mut xattrs) };

        // create the colormap
        let colormap = unsafe {
            xlib::XCreateColormap(dpy.raw()?.as_mut(), win, xattrs.visual, xlib::AllocAll)
        };
        let colormap = ColorMap::from_raw(colormap, &dpy)?;

        Ok(Self { win, dpy, gc, colormap })
    }

    /// Get the graphics context for this window.
    #[inline]
    pub fn gc(&self) -> NonNull<_XGC> {
        self.gc
    }

    /// Get the color map for this window.
    #[inline]
    pub fn colormap(&self) -> &ColorMap {
        &self.colormap
    }

    /// Get the inner number representing the window.
    #[inline]
    pub fn window(&self) -> xlib::Window {
        self.win
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if let Ok(mut d) = self.dpy.raw() {
            unsafe {
                xlib::XFreeGC(d.as_mut(), self.gc.as_mut());
                xlib::XDestroyWindow(d.as_mut(), self.win);
            };
        }
    }
}
