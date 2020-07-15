/* -----------------------------------------------------------------------------------
 * src/window/internal/mod.rs - Define a wrapper around the backended internals
 *                              for the window.
 * beetle - Pull-based GUI framework
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

use super::Window;
use crate::{EventType, Instance, Pixel, Texture};
use alloc::boxed::Box;
use euclid::{Point2D, Size2D};

/// Methods that every internal window is expected to provide.
pub trait GenericInternalWindow {
    /// Show this window.
    fn show(&self) -> crate::Result<()>;
    /// Set the size of this window.
    fn set_size(&self, bounds: Size2D<u32, Pixel>) -> crate::Result<()>;
    /// Set the text associated with the window.
    fn set_text(&self, text: &str) -> crate::Result<()>;
    /// Receive events of a certain types.
    fn receive_events(&self, events: &[EventType]) -> crate::Result<()>;
}

#[cfg(target_os = "linux")]
pub mod flutter;
#[cfg(windows)]
pub mod porc;

/// Internal storage for the window backend.
pub enum InternalWindow {
    #[cfg(windows)]
    Porcupine(porc::PorcIW),
    #[cfg(target_os = "linux")]
    Flutter(flutter::FlutterIW),
}

impl InternalWindow {
    /// Get a reference to the internal window.
    pub fn generic(&self) -> &dyn GenericInternalWindow {
        match self {
            #[cfg(windows)]
            Self::Porcupine(ref p) => p,
            #[cfg(target_os = "linux")]
            Self::Flutter(ref f) => f,
        }
    }
}
