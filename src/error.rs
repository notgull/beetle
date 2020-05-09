/* -----------------------------------------------------------------------------------
 * src/error.rs - An error type that encompasses all errors created by Beetle.
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

use crate::FreetypeError;
use std::{
    cell::{BorrowError, BorrowMutError},
    ffi::NulError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Msg(String),
    #[error("{0}")]
    StaticMsg(&'static str),
    #[error("{0}")]
    Nul(#[from] NulError),
    #[error("{0}")]
    Freetype(#[from] FreetypeError),
    #[error("{0}")]
    Borrow(#[from] BorrowError),
    #[error("{0}")]
    BorrowMut(#[from] BorrowMutError),

    // semantic errors
    #[error("Cannot perform operation with raw internal object")]
    RawInternalInability,
    #[error("Widget reference points to dropped widget")]
    DroppedWidget,

    // X11 errors
    #[error("Unable to open display")]
    UnableToOpenDisplay,
    #[error("The display object was destroyed")]
    DroppedDisplay,
    #[error("Expected LGuiObject to be a window")]
    ExpectedWindow,
    #[error("The root window cannot have its parent reassigned")]
    RootWindowCannotReassignParent,
    #[error("A window's parent cannot be a sub-element")]
    NoSubelementParent,
    #[error("Unable to create the X11 Graphics Context")]
    NoXGC,
}
