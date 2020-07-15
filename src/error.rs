/* -----------------------------------------------------------------------------------
 * src/error.rs - A common error type for all Beetle functions.
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

use alloc::string::String;
use core::{fmt, num::TryFromIntError};
#[cfg(target_os = "linux")]
use flutterbug::FlutterbugError;
use ordered_float::FloatIsNan;
#[cfg(windows)]
use porcupine::Error as PorcupineError;

/// Things that can cause a color to be invalid.
#[derive(Debug)]
pub enum InvalidColor {
    OutOfRange(f32),
    FoundNan(FloatIsNan),
}

impl fmt::Display for InvalidColor {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidColor::OutOfRange(i) => write!(f, "Expected float {} to be between -1.0f and 1.0f", i),
            InvalidColor::FoundNan(ref fnan) => fmt::Display::fmt(fnan, f),
        }
    }
}

/// Common error type returned by Beetle functions.
#[derive(Debug)]
pub enum Error {
    Unreachable,
    StaticMsg(&'static str),
    Msg(String),
    #[cfg(target_os = "linux")]
    Flutter(FlutterbugError),
    #[cfg(windows)]
    Porc(PorcupineError),
    TryFromInt(TryFromIntError),
    InvalidColor(InvalidColor),

    WindowNotFound,
    KeysymNotFound,
    WindowIDNoDowncast,
    UnableToWrite,
    UnableToRead,
    WindowMismatch,
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(target_os = "linux")]
        {
            if let Self::Flutter(ref flutter) = self {
                return fmt::Display::fmt(flutter, f);
            }
        }

        #[cfg(windows)]
        {
            if let Self::Porc(ref porc) = self {
                return fmt::Display::fmt(porc, f);
            }
        }

        match self {
            Self::Unreachable => unreachable!(),
            Self::StaticMsg(s) => f.pad(s),
            Self::Msg(ref s) => f.pad(s),
            Self::TryFromInt(ref i) => fmt::Display::fmt(i, f),
            Self::WindowNotFound => f.pad("Unable to find window in window mappings"),
            Self::KeysymNotFound => f.pad("Unable to find key symbol corresponding to input"),
            Self::WindowIDNoDowncast => f.pad("Window ID did not downcast to a valid element"),
            Self::UnableToWrite => f.pad("Unable to write to RwLock"),
            Self::UnableToRead => f.pad("Unable to read from RwLock"),
            Self::WindowMismatch => f.write_str("Windows are of two different types"),
            Self::InvalidColor(ref i) => fmt::Display::fmt(i, f),
            _ => unreachable!(),
        }
    }
}

#[cfg(target_os = "linux")]
impl From<FlutterbugError> for Error {
    #[inline]
    fn from(fe: FlutterbugError) -> Self {
        Self::Flutter(fe)
    }
}

#[cfg(windows)]
impl From<PorcupineError> for Error {
    #[inline]
    fn from(p: PorcupineError) -> Self {
        Self::Porc(p)
    }
}

impl From<TryFromIntError> for Error {
    #[inline]
    fn from(tfie: TryFromIntError) -> Self {
        Self::TryFromInt(tfie)
    }
}

impl From<FloatIsNan> for Error {
    #[inline]
    fn from(fin: FloatIsNan) -> Self {
        Self::InvalidColor(InvalidColor::FoundNan(fin))
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Result type that returns beetle::Error to make things more conveinent.
pub type Result<T> = core::result::Result<T, Error>;
