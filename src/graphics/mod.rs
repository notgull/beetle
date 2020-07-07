/*
 * src/graphics/mod.rs - Graphics object and API.
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

use crate::{Color, Window};
use alloc::boxed::Box;
use core::fmt;
use euclid::default::Point2D;

#[cfg(target_os = "linux")]
mod flutter;
#[cfg(windows)]
mod porc;
#[cfg(target_os = "linux")]
pub(crate) use flutter::*;
#[cfg(windows)]
pub(crate) use porc::*;

/// The internal graphics object. This is loaded into the Graphics object and used
/// for its methods.
pub trait InternalGraphics {
    /// Set the foreground color.
    fn set_foreground(&self, clr: Color) -> crate::Result<()>;

    /// Set the background color.
    fn set_background(&self, clr: Color) -> crate::Result<()>;

    /// Draw a line from one point to another, using the foreground color.
    fn draw_line(&self, p1: Point2D<u32>, p2: Point2D<u32>) -> crate::Result<()>;
}

// storage object for internal graphics object
enum GraphicsStorage {
    #[cfg(target_os = "linux")]
    Flutter(FlutterbugGraphics),
    #[cfg(windows)]
    Porc(PorcupineGraphics),
    Other(Box<dyn InternalGraphics>),
}

impl GraphicsStorage {
    #[inline]
    fn graphics(&self) -> &dyn InternalGraphics {
        #[cfg(target_os = "linux")]
        if let Self::Flutter(ref fl) = self {
            return fl;
        }

        #[cfg(windows)]
        if let Self::Porc(ref p) = self {
            return p;
        }

        if let Self::Other(ref o) = self {
            return &**o;
        }

        unimplemented!()
    }
}

/// The graphics object used in painting operations.
#[repr(transparent)]
pub struct Graphics(GraphicsStorage);

impl fmt::Debug for Graphics {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Graphics")
    }
}

impl Graphics {
    /// Create a new graphics object from an object implementing InternalGraphics.
    #[inline]
    pub fn new(internal: Box<dyn InternalGraphics>) -> Graphics {
        Self(GraphicsStorage::Other(internal))
    }

    /// Create a graphics object based on a window.
    #[inline]
    pub fn from_window(window: &Window) -> crate::Result<Graphics> {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "linux")] {
                Ok(Self(GraphicsStorage::Flutter(FlutterbugGraphics::new(window)?)))
            } else {
                unimplemented!()
            }
        }
    }

    /// Get a reference to the internal graphics object.
    #[inline]
    pub fn graphics(&self) -> &dyn InternalGraphics {
        self.0.graphics()
    }
}
