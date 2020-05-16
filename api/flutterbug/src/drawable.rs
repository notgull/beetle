/* -----------------------------------------------------------------------------------
 * api/flutterbug/src/drawable.rs - Base trait for items that can be drawed upon. They
 *                                  are expected to hold references to their own GCs.
 *                                  This file also defines a wrapper for the GC
 *                                  struct.
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

use super::{DisplayReference, FlutterbugError, GenericDisplay, HasXID};
use euclid::default::{Point2D, Rect};
use std::{
    fmt,
    os::raw::c_int,
    ptr::NonNull,
    sync::{Arc, Weak},
};
use x11::xlib::{self, _XGC};

/// A graphics context.
pub struct GraphicsContext {
    raw: Arc<NonNull<_XGC>>,
    dpy: DisplayReference,
    is_default: bool,
}

impl fmt::Debug for GraphicsContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}X11 Graphics Context",
            if self.is_default { "Default " } else { "" }
        )
    }
}

impl Clone for GraphicsContext {
    fn clone(&self) -> Self {
        Self::from_raw(self.raw.clone(), self.dpy.clone(), self.is_default)
    }
}

impl Drop for GraphicsContext {
    fn drop(&mut self) {
        // we shouldn't call XFreeGC if we're using the default context
        if !self.is_default {
            if let Ok(mut d) = self.dpy.raw() {
                unsafe { xlib::XFreeGC(d.as_mut(), self.raw.as_ptr()) };
            }
        }
    }
}

impl GraphicsContext {
    #[inline]
    pub(crate) fn from_raw(
        raw: Arc<NonNull<_XGC>>,
        dpy: DisplayReference,
        is_default: bool,
    ) -> Self {
        Self {
            raw,
            dpy,
            is_default,
        }
    }
}

/// A reference to a graphics context.
pub struct GraphicsContextReference {
    raw: Weak<NonNull<_XGC>>,
    dpy: DisplayReference,
    is_default: bool,
}

impl fmt::Debug for GraphicsContextReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}X11 Graphics Context Reference",
            if self.is_default { "Default " } else { "" }
        )
    }
}

impl Clone for GraphicsContextReference {
    fn clone(&self) -> Self {
        Self::from_raw(self.raw.clone(), self.dpy.clone(), self.is_default)
    }
}

impl GraphicsContextReference {
    #[inline]
    pub(crate) fn from_raw(
        raw: Weak<NonNull<_XGC>>,
        dpy: DisplayReference,
        is_default: bool,
    ) -> Self {
        Self {
            raw,
            dpy,
            is_default,
        }
    }

    #[inline]
    pub(crate) fn dpy(&self) -> &DisplayReference {
        &self.dpy
    }
}

/// A trait implemented by both GraphicsContext and GraphicsContextReference,
/// in order to abstract them out.
pub trait GenericGraphicsContext: fmt::Debug + Clone {
    /// Get a reference to this graphics context.
    fn reference(&self) -> GraphicsContextReference;
    /// Get the raw pointer to the graphics context.
    fn raw(&self) -> Result<NonNull<_XGC>, FlutterbugError>;
    /// Tell if this is the default context.
    fn is_default(&self) -> bool;
}

impl GenericGraphicsContext for GraphicsContext {
    fn reference(&self) -> GraphicsContextReference {
        GraphicsContextReference::from_raw(
            Arc::downgrade(&self.raw),
            self.dpy.clone(),
            self.is_default,
        )
    }

    fn raw(&self) -> Result<NonNull<_XGC>, FlutterbugError> {
        Ok(*self.raw)
    }

    fn is_default(&self) -> bool {
        self.is_default
    }
}

impl GenericGraphicsContext for GraphicsContextReference {
    fn reference(&self) -> GraphicsContextReference {
        self.clone()
    }
    fn raw(&self) -> Result<NonNull<_XGC>, FlutterbugError> {
        Ok(*self
            .raw
            .upgrade()
            .ok_or_else(|| FlutterbugError::GCWasDropped)?)
    }
    fn is_default(&self) -> bool {
        self.is_default
    }
}

/// Objects that can be drawed upon.
pub trait Drawable: HasXID + fmt::Debug {
    /// Get a reference to the graphics context that this item has stored.
    fn gc_ref(&self) -> GraphicsContextReference;
    /// Draw a string on this object.
    fn draw_string(&self, origin: Point2D<u32>, text: String) -> Result<(), FlutterbugError> {
        let gc = self.gc_ref();
        let tlen = text.len();
        unsafe {
            xlib::XDrawString(
                gc.dpy().raw()?.as_mut(),
                self.xid(),
                gc.raw()?.as_mut(),
                origin.x as c_int,
                origin.y as c_int,
                super::to_cstring(text)?,
                tlen as c_int,
            )
        };
        Ok(())
    }
}
