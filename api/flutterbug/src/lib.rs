/* -----------------------------------------------------------------------------------
 * api/flutterbug/src/lib.rs - Root of the Flutterbug library, for safe X11 bindings.
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

//! X11 wrapper library.

use euclid::default::Rect;
use std::{
    ffi::CString,
    fmt, mem,
    os::raw::{c_char, c_int, c_uint, c_ulong},
    ptr::{self, NonNull},
    sync::{Arc, Weak},
};
use x11::xlib::{self, XID, _XGC};

pub mod color;
pub use color::*;
pub mod context;
pub use context::*;
pub mod drawable;
pub use drawable::*;
pub mod error;
pub use error::*;
pub mod event;
pub use event::*;
mod screen;
pub use screen::*;
pub mod window;
pub use window::*;

/// Utility function to convert a String into an ASCII *mut c_char
#[inline]
pub(crate) unsafe fn to_cstring(s: String) -> Result<*mut c_char, FlutterbugError> {
    Ok(CString::new(s)?.into_raw())
}

/// A trait that represents that something can be transformed into an XID.
pub trait HasXID {
    /// Get the XID for this instance.
    fn xid(&self) -> XID;
}

impl HasXID for XID {
    fn xid(&self) -> XID {
        *self
    }
}

/// The X11 display. This is the context object used for the X11 window.
///
/// Note: This object is not clonable. Use the reference() method to get
/// a cheap reference to this object.
pub struct Display {
    raw: Arc<NonNull<xlib::Display>>,
}

// make sure it can be debugged
impl fmt::Debug for Display {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11 Display Object")
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { xlib::XCloseDisplay(self.raw.as_ptr()) };
    }
}

impl Display {
    /// Create a new Display. This will call the XOpenDisplay function and
    /// store the result in an Arc. If XOpenDisplay returns null, the
    /// UnableToOpenDisplay error is returned instead.
    #[inline]
    pub fn new() -> Result<Self, FlutterbugError> {
        let display_ptr = unsafe { xlib::XOpenDisplay(ptr::null()) };
        match NonNull::new(display_ptr) {
            Some(dpy) => Ok(Self { raw: Arc::new(dpy) }),
            None => Err(FlutterbugError::UnableToOpenDisplay),
        }
    }

    /// Since the Display object is a cheap wrapper around the Display pointer,
    /// we can use it to forward calls to the Arc<> once we upgrade a Weak<> to
    /// it. This just creates the wrapper.
    #[inline]
    pub(crate) fn from_raw(raw: Arc<NonNull<xlib::Display>>) -> Self {
        Self { raw }
    }
}

/// A reference to the X11 display. Unlike the Display object, this is
/// clonable. However, it will also decay if its parent Display object
/// is dropped.
pub struct DisplayReference {
    reference: Weak<NonNull<xlib::Display>>,
}

// make sure it can be debuged
impl fmt::Debug for DisplayReference {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11 Display Reference")
    }
}

impl Clone for DisplayReference {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            reference: self.reference.clone(),
        }
    }
}

impl DisplayReference {
    #[inline]
    pub(crate) fn from_ref(reference: Weak<NonNull<xlib::Display>>) -> Self {
        Self { reference }
    }

    /// Get the Display object that this DisplayReference refers to.
    #[inline]
    pub fn upgrade(&self) -> Result<Display, FlutterbugError> {
        Ok(Display::from_raw(
            self.reference
                .upgrade()
                .ok_or_else(|| FlutterbugError::DisplayWasDropped)?,
        ))
    }
}

/// This trait is applied to both Display and DisplayReference to ensure you can do
/// the same things with both.
///
/// Note: All methods return a Result<T, FlutterbugError> since upgrading the reference
/// to a full object can generate an error if the real Display is already dropped.
pub trait GenericDisplay: fmt::Debug {
    /// Create a reference to this object.
    fn reference(&self) -> DisplayReference;
    /// Get the pointer to the raw Display object.
    fn raw(&self) -> Result<NonNull<xlib::Display>, FlutterbugError>;
    /// Get the default screen for this instance.
    fn default_screen(&self) -> Result<Screen, FlutterbugError> {
        Ok(Screen::new(unsafe {
            xlib::XDefaultScreen(self.raw()?.as_mut())
        }))
    }
    /// Get the black pixel for the default screen.
    fn black_pixel(&self) -> Result<Color, FlutterbugError> {
        Ok(Color::PixelID(unsafe {
            xlib::XBlackPixel(self.raw()?.as_mut(), self.default_screen()?.value())
        }))
    }
    /// Get the white pixel for the default screen.
    fn white_pixel(&self) -> Result<Color, FlutterbugError> {
        Ok(Color::PixelID(unsafe {
            xlib::XWhitePixel(self.raw()?.as_mut(), self.default_screen()?.value())
        }))
    }
    /// Create a simple window from this display.
    fn create_simple_window(
        &self,
        parent: Option<&Window>,
        bounds: Rect<u32>,
        border_width: u32,
        border_color: Color,
        background_color: Color,
    ) -> Result<Window, FlutterbugError> {
        macro_rules! test_color {
            ($cname: ident) => {
                if $cname != self.black_pixel()? || $cname != self.white_pixel()? {
                    return Err(FlutterbugError::Msg(format!(
                        "{} must be either black or white",
                        &stringify!($cname)
                    )));
                }
            };
        }

        test_color!(border_color);
        test_color!(background_color);

        let win = unsafe {
            xlib::XCreateSimpleWindow(
                self.raw()?.as_mut(),
                match parent {
                    Some(p) => p.window(),
                    None => xlib::XRootWindow(self.raw()?.as_mut(), self.default_screen()?.value()),
                },
                bounds.origin.x as c_int,
                bounds.origin.y as c_int,
                bounds.size.width as c_uint,
                bounds.size.height as c_uint,
                border_width as c_uint,
                border_color.pixel_id(),
                background_color.pixel_id(),
            )
        };
        Window::from_raw(win, self.reference())
    }
    /// Create a context using this connection.
    fn create_context(&self) -> Result<Context, FlutterbugError> {
        Ok(Context::from_dpy(self.reference()))
    }
}

impl GenericDisplay for Display {
    #[inline]
    fn reference(&self) -> DisplayReference {
        DisplayReference::from_ref(Arc::downgrade(&self.raw))
    }
    #[inline]
    fn raw(&self) -> Result<NonNull<xlib::Display>, FlutterbugError> {
        Ok(*self.raw)
    }
}

// just forward calls to the inner Display
impl GenericDisplay for DisplayReference {
    #[inline]
    fn reference(&self) -> DisplayReference {
        self.clone()
    }
    #[inline]
    fn raw(&self) -> Result<NonNull<xlib::Display>, FlutterbugError> {
        self.upgrade()?.raw()
    }
}

/// Traits that should be imported in order to ensure the function of the library.
pub mod prelude {
    use super::{DerivesAnEvent, DerivesEvent, GenericDisplay, GenericGraphicsContext, HasXID};
}
