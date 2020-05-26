/* -----------------------------------------------------------------------------------
 * src/event.rs - This file should define a basic, QT-like slot/signal event system
 *                for Beetle. The "Slot" object should store a vector of boxed
 *                functions that can be called with a specific set of event arguments.
 *                The "Signal" object should consist of a boxed list of event
 *                arguments that can activate a Slot, causing it to call of of its
 *                stored functions.
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

use euclid::default::{Point2D, Rect};
use std::{boxed::Box, fmt};

/// A signal. This can activate a certain Beetle event.
pub enum Signal {
    /// The window has been created.
    Created(()),
    /// The window has been repainted.
    Repaint(()),
    /// The window's bounds have been changed.
    BoundsChanged((Rect<u32>, Rect<u32>)),
}

/// A specific type of signal.
pub trait TypedSignal<Args> {
    /// Get the args of this signal.
    fn args(self) -> Option<Args>;
}

// macro for implementing TypedSignal
macro_rules! impl_tsignal {
    ($ename: ident, $args: ty) => {
        impl TypedSignal<$args> for Signal {
            fn args(self) -> Option<$args> {
                match self {
                    Self::$ename(a) => Some(a),
                    _ => None,
                }
            }
        }
    };
}

impl_tsignal! {Created, ()}
impl_tsignal! {BoundsChanged, (Rect<u32>, Rect<u32>)}

/// A slot. This holds handlers.
pub struct Slot<Args> {
    handlers: Vec<Box<dyn Fn(&Args) -> Result<(), crate::Error>>>,
}

impl<Args> Slot<Args> {
    /// Activate all of the handlers in this slot.
    pub fn activate(&self, signal: impl TypedSignal<Args>) -> Result<(), crate::Error> {
        let args = signal
            .args()
            .ok_or_else(|| crate::Error::SignalArgumentMismatch)?;
        self.handlers.iter().try_for_each(|h| h(&args))
    }
}
