/* -----------------------------------------------------------------------------------
 * src/lib.rs - Root of the Beetle library.
 * porcupine - Safe wrapper around the graphical parts of Win32.
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

//! Beetle is a GUI library that aims to use a pull-based event system, rather than the push-based
//! event system that most modern GUI frameworks use.
//!
//! Beetle is built upon the idea that control over the event loop should belong to the programmer,
//! rather than the framework.

#![feature(trait_alias)]
#![no_std]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
extern crate std as alloc;

//#![cfg_attr(target_os = "linux", feature("flutterbug"))]
//#![cfg_attr(windows, feature("porcupine"))]
pub mod color;
pub mod error;
pub mod event;
pub mod instance;
pub mod keyboard;
pub mod mouse;
pub mod ro_mmg;
pub(crate) mod take_vec;
pub mod texture;
pub mod window;
pub(crate) mod wndproc;

pub use color::*;
pub use error::*;
pub use event::*;
pub use instance::*;
pub use keyboard::*;
pub use mouse::*;
#[cfg(feature = "std")]
pub use ro_mmg::*;
pub use texture::*;
pub use window::*;

// helper for mutex operations
// if the std feature is activated, we can use parking_lot mutexes
// otherwise, we fall back to spin mutexes
pub(crate) mod mutexes {
    #[cfg(feature = "std")]
    pub use parking_lot::*;
    #[cfg(not(feature = "std"))]
    pub use spin::*;
}

pub mod prelude {}
