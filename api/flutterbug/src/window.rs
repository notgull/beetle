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

use super::{
    to_cstring, ColorMap, DisplayReference, Drawable, FlutterbugError, GenericDisplay,
    GenericGraphicsContext, GraphicsContext, GraphicsContextReference, HasXID,
};
use euclid::default::{Point2D, Rect, Size2D};
use std::{
    ffi::CString,
    mem,
    os::raw::{c_int, c_uint, c_ulong, c_ushort},
    ptr::{self, NonNull},
    sync::Arc,
};
use x11::xlib::{self, _XGC};

bitflags::bitflags! {
    #[doc = "Flags for inserting values into an XSetWindowAttributes."]
    pub struct ChangeWindow : c_ulong {
        const X = xlib::CWX as c_ulong;
        const Y = xlib::CWY as c_ulong;
        const WIDTH = xlib::CWWidth as c_ulong;
        const HEIGHT = xlib::CWHeight as c_ulong;
        const CURSOR = xlib::CWCursor;
        const SIBLING = xlib::CWSibling as c_ulong;
        const COLOR_MAP = xlib::CWColormap;
        const BACK_PIXEL = xlib::CWBackPixel;
        const EVENT_MASK = xlib::CWEventMask;
        const SAVE_UNDER = xlib::CWSaveUnder;
        const STACK_MODE = xlib::CWStackMode as c_ulong;
        const BACK_PIXMAP = xlib::CWBackPixmap;
        const BIT_GRAVITY = xlib::CWBitGravity;
        const WIN_GRAVITY = xlib::CWWinGravity;
        const BORDER_PIXEL = xlib::CWBorderPixel;
        const BORDER_WIDTH = xlib::CWBorderWidth as c_ulong;
        const BACKING_PIXEL = xlib::CWBackingPixel;
        const BACKING_STORE = xlib::CWBackingStore;
        const BORDER_PIXMAP = xlib::CWBorderPixmap;
        const BACKING_PLANES = xlib::CWBackingPlanes;
        const DONT_PROPAGATE = xlib::CWDontPropagate;
        const OVERRIDE_REDIRECT = xlib::CWOverrideRedirect;
    }
}

/// An X11 window. This usually represents a rectangle of pixels on the screen.
#[derive(Debug)]
pub struct Window {
    win: xlib::Window,
    dpy: DisplayReference,
    // window should also store a reference to its GC and Colormap
    gc: GraphicsContext,
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
        let gc = GraphicsContext::from_raw(Arc::new(gc), dpy.clone(), false);

        // get the pointer to the visual item
        let mut xattrs: xlib::XWindowAttributes = unsafe { mem::zeroed() };
        unsafe { xlib::XGetWindowAttributes(dpy.raw()?.as_mut(), win, &mut xattrs) };

        // create the colormap
        let colormap = unsafe {
            xlib::XCreateColormap(dpy.raw()?.as_mut(), win, xattrs.visual, xlib::AllocAll)
        };
        let colormap = ColorMap::from_raw(colormap, &dpy)?;

        Ok(Self {
            win,
            dpy,
            gc,
            colormap,
        })
    }

    /// Get the graphics context for this window.
    #[inline]
    pub fn gc(&self) -> &GraphicsContext {
        &self.gc
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

    /// Map this window, as either raised or not raised.
    #[inline]
    pub fn map(&self, raised: bool) -> Result<(), FlutterbugError> {
        if raised {
            unsafe { xlib::XMapRaised(self.dpy.raw()?.as_mut(), self.win) };
        } else {
            unsafe { xlib::XMapWindow(self.dpy.raw()?.as_mut(), self.win) };
        }

        // TODO: error handling
        Ok(())
    }

    /// Unmap this window.
    #[inline]
    pub fn unmap(&self) -> Result<(), FlutterbugError> {
        unsafe { xlib::XUnmapWindow(self.dpy.raw()?.as_mut(), self.win) };
        // TODO: error handling
        Ok(())
    }

    /// Get the window attributes for this window.
    #[inline]
    pub fn window_attributes(&self) -> Result<xlib::XWindowAttributes, FlutterbugError> {
        let mut xattrs: xlib::XWindowAttributes = unsafe { mem::zeroed() };
        unsafe { xlib::XGetWindowAttributes(self.dpy.raw()?.as_mut(), self.win, &mut xattrs) };
        // TODO: check window
        Ok(xattrs)
    }

    /// Set the window attributes for this window.
    #[inline]
    pub fn set_window_attributes(
        &self,
        mut xset: xlib::XSetWindowAttributes,
        changes: ChangeWindow,
    ) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XChangeWindowAttributes(
                self.dpy.raw()?.as_mut(),
                self.win,
                changes.bits(),
                &mut xset,
            )
        };
        Ok(())
    }

    /// Move the window to a different location.
    #[inline]
    pub fn set_position(&self, pt: Point2D<u32>) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XMoveWindow(
                self.dpy.raw()?.as_mut(),
                self.win,
                pt.x as c_int,
                pt.y as c_int,
            )
        };
        Ok(())
    }

    /// Resize the window to use a different size.
    #[inline]
    pub fn resize(&self, sz: Size2D<u32>) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XResizeWindow(
                self.dpy.raw()?.as_mut(),
                self.win,
                sz.width as c_uint,
                sz.height as c_uint,
            )
        };
        Ok(())
    }

    /// Change the window's bounds overall.
    #[inline]
    pub fn set_bounds(&self, bnds: Rect<u32>) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XMoveResizeWindow(
                self.dpy.raw()?.as_mut(),
                self.win,
                bnds.origin.x as c_int,
                bnds.origin.y as c_int,
                bnds.size.width as c_uint,
                bnds.size.height as c_uint,
            )
        };
        Ok(())
    }

    /// Raise this window to the top of the stack.
    #[inline]
    pub fn raise(&self) -> Result<(), FlutterbugError> {
        unsafe { xlib::XRaiseWindow(self.dpy.raw()?.as_mut(), self.win) };
        Ok(())
    }

    /// Store the name of this window.
    #[inline]
    pub fn store_name(&self, name: String) -> Result<(), FlutterbugError> {
        let cstr = unsafe { to_cstring(name)? };
        unsafe { xlib::XStoreName(self.dpy.raw()?.as_mut(), self.win, cstr) };
        // take back the cstr to prevent a memory leak
        let _ = unsafe { CString::from_raw(cstr) };
        Ok(())
    }

    /// Set standard properties for this window.
    #[inline]
    pub fn set_standard_properties(
        &mut self,
        window_name: Option<String>,
        icon_name: Option<String>,
    ) -> Result<(), FlutterbugError> {
        Ok(())
    }
}

impl HasXID for Window {
    #[inline]
    fn xid(&self) -> xlib::XID {
        self.win
    }
}

impl Drawable for Window {
    #[inline]
    fn gc_ref(&self) -> GraphicsContextReference {
        self.gc().reference()
    }
}

// macro for using the set function
/*macro_rules! set_winattrs {
    ($($prop: ident = $val: expr),* $($flag: expr)|*) => {
        let mut xset = xlib::XSetWindowAttributes {
            $($prop: $val),*
            ..unsafe { std::mem::zeroed() }
        };

        self.set_window_attributes(xset, $($flag)|*)
    }
}*/

impl Drop for Window {
    fn drop(&mut self) {
        if let Ok(mut d) = self.dpy.raw() {
            unsafe {
                xlib::XFreeGC(d.as_mut(), self.gc.raw().unwrap().as_mut());
                xlib::XDestroyWindow(d.as_mut(), self.win);
            };
        }
    }
}
