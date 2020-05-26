/* -----------------------------------------------------------------------------------
 * src/color.rs - This file should define a simple color structure to store 3 bytes
 *                worth of color information.
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

use approx::abs_diff_eq;
use std::fmt;

/// Possible elements of a color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorPart {
    Red,
    Green,
    Blue,
    Alpha,
}

impl fmt::Display for ColorPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Red => write!(f, "Red"),
            Self::Green => write!(f, "Green"),
            Self::Blue => write!(f, "Blue"),
            Self::Alpha => write!(f, "Alpha"),
        }
    }
}

/// A color, consisting of the four components in 32-bit float notation.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

/// Basic function to tell if a float is between 0.0 and 1.0
#[inline]
fn is_in_range(val: f32) -> bool {
    val >= 0.0 && val <= 1.0
}

/// Check four values.
#[inline]
fn check_four(f1: f32, f2: f32, f3: f32, f4: f32) -> bool {
    match (
        is_in_range(f1),
        is_in_range(f2),
        is_in_range(f3),
        is_in_range(f4),
    ) {
        (true, true, true, true) => true,
        _ => false,
    }
}

impl Color {
    /// Create a new three-entry color.
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Option<Self> {
        Self::with_alpha(r, g, b, 1.0)
    }

    /// Create a new four-entry color.
    #[inline]
    pub fn with_alpha(r: f32, g: f32, b: f32, a: f32) -> Option<Self> {
        // check if values are in range
        if check_four(r, g, b, a) {
            Some(Self { r, g, b, a })
        } else {
            None
        }
    }

    /// Create a new four-entry color, without checking if it is out of range.
    #[inline]
    pub unsafe fn with_alpha_unchecked(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a transparent color.
    #[inline]
    pub fn transparent() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }

    /// Tell if the color is transparent.
    #[inline]
    pub fn is_transparent(&self) -> bool {
        abs_diff_eq!(self.a, 0.0f32)
    }

    /// Get the red component.
    #[inline]
    pub fn r(&self) -> f32 {
        self.r
    }

    /// Set the red component.
    #[inline]
    pub fn set_r(&mut self, r: f32) -> Result<(), crate::Error> {
        if is_in_range(r) {
            self.r = r;
            Ok(())
        } else {
            Err(crate::Error::ColorOutOfRange(r))
        }
    }

    /// Set the red component without checking if it is in range.
    #[inline]
    pub unsafe fn set_r_unchecked(&mut self, r: f32) {
        self.r = r;
    }

    /// Get the green component.
    #[inline]
    pub fn g(&self) -> f32 {
        self.g
    }

    /// Set the green component.
    #[inline]
    pub fn set_g(&mut self, g: f32) -> Result<(), crate::Error> {
        if is_in_range(g) {
            self.g = g;
            Ok(())
        } else {
            Err(crate::Error::ColorOutOfRange(g))
        }
    }

    /// Set the green component without checking if it is in range.
    #[inline]
    pub unsafe fn set_g_unchecked(&mut self, g: f32) {
        self.g = g;
    }

    /// Get the blue component.
    #[inline]
    pub fn b(&self) -> f32 {
        self.b
    }

    /// Set the blue component.
    #[inline]
    pub fn set_b(&mut self, b: f32) -> Result<(), crate::Error> {
        if is_in_range(b) {
            self.b = b;
            Ok(())
        } else {
            Err(crate::Error::ColorOutOfRange(b))
        }
    }

    /// Set the red component without checking if it is in range.
    #[inline]
    pub unsafe fn set_b_unchecked(&mut self, b: f32) {
        self.b = b;
    }

    /// If this color is partially transparent, merge it with a non-transparent color.
    #[inline]
    pub fn merge_down(&self, other: &Color) -> Result<Color, crate::Error> {
        // if we are transparent or solid, there are shortcuts to take!
        match (
            abs_diff_eq!(other.a, 1.0),
            abs_diff_eq!(self.a, 0.0),
            abs_diff_eq!(self.a, 1.0),
        ) {
            (false, _, _) => Err(crate::Error::NeedsSolidMergeDown),
            (true, true, _) => Ok(*other),    // we are transparent
            (true, false, true) => Ok(*self), // we are solid
            _ => {
                let other_mult = 1.0 - self.a;
                let r = (self.r * self.a) + (other.r * other_mult);
                let g = (self.g * self.a) + (other.g * other_mult);
                let b = (self.b * self.a) + (other.b * other_mult);
                Ok(Self { r, g, b, a: 1.0 })
            }
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}
