/* -----------------------------------------------------------------------------------
 * src/color.rs - A basic structure for colors.
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

use crate::InvalidColor;
use ordered_float::NotNan;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Color {
    r: NotNan<f32>,
    g: NotNan<f32>,
    b: NotNan<f32>,
    a: NotNan<f32>,
}

#[inline]
fn checked_cnvrt(fe: f32) -> crate::Result<NotNan<f32>> {
    if -1.0 <= fe && fe <= 1.0 {
        Ok(NotNan::new(fe)?)
    } else {
        Err(crate::Error::InvalidColor(InvalidColor::OutOfRange(fe)))
    }
}

impl Color {
    /// Create a new color. This function checks for NaN values, and returns an error if it finds one.
    #[inline]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> crate::Result<Self> {
        Ok(Self {
            r: checked_cnvrt(r)?,
            g: checked_cnvrt(g)?,
            b: checked_cnvrt(b)?,
            a: checked_cnvrt(a)?,
        })
    }

    /// Create a new color without checking for NaN values.
    #[inline]
    pub unsafe fn new_no_nan_check(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: NotNan::unchecked_new(r),
            g: NotNan::unchecked_new(g),
            b: NotNan::unchecked_new(b),
            a: NotNan::unchecked_new(a),
        }
    }

    /// Get the red component of this color.
    #[inline]
    pub fn r(&self) -> f32 {
        self.r.into_inner()
    }

    /// Get the green component of this color.
    #[inline]
    pub fn g(&self) -> f32 {
        self.g.into_inner()
    }

    /// Get the blue component of this color.
    #[inline]
    pub fn b(&self) -> f32 {
        self.b.into_inner()
    }

    /// Get the alpha component of this color.
    #[inline]
    pub fn a(&self) -> f32 {
        self.a.into_inner()
    }

    /// Set the red component of this color.
    #[inline]
    pub fn set_r(&mut self, val: f32) -> crate::Result<()> {
        self.r = checked_cnvrt(val)?;
        Ok(())
    }

    /// Set the red component of this color without checking for invalid values.
    #[inline]
    pub unsafe fn set_r_unchecked(&mut self, val: f32) {
        self.r = NotNan::unchecked_new(val);
    }

    /// Set the green component of this color.
    #[inline]
    pub fn set_g(&mut self, val: f32) -> crate::Result<()> {
        self.g = checked_cnvrt(val)?;
        Ok(())
    }

    /// Set the green component of this color without checking for invalid values.
    #[inline]
    pub unsafe fn set_g_unchecked(&mut self, val: f32) {
        self.g = NotNan::unchecked_new(val);
    }

    /// Set the blue component of this color.
    #[inline]
    pub fn set_b(&mut self, val: f32) -> crate::Result<()> {
        self.b = checked_cnvrt(val)?;
        Ok(())
    }

    /// Set the blue component of this color without checking for invalid values.
    #[inline]
    pub unsafe fn set_b_unchecked(&mut self, val: f32) {
        self.b = NotNan::unchecked_new(val);
    }

    /// Set the alpha component of this color.
    #[inline]
    pub fn set_a(&mut self, val: f32) -> crate::Result<()> {
        self.a = checked_cnvrt(val)?;
        Ok(())
    }

    /// Set the alpha component of this color without checking for invalid values.
    #[inline]
    pub unsafe fn set_a_unchecked(&mut self, val: f32) {
        self.a = NotNan::unchecked_new(val);
    }

    /// Create a new color from RGB bytes.
    #[inline]
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        macro_rules! cnvrt_u8_f32 {
            ($val: expr) => {{
                ($val as f32) / (core::u8::MAX as f32)
            }};
        }

        unsafe { Self::new_no_nan_check(cnvrt_u8_f32!(r), cnvrt_u8_f32!(g), cnvrt_u8_f32!(b), cnvrt_u8_f32!(a)) }
    }
}

/// Several common colors.
pub mod colors {
    use super::Color;

    pub fn black() -> Color {
        unsafe { Color::new_no_nan_check(0.0, 0.0, 0.0, 1.0) }
    }
    pub fn white() -> Color {
        unsafe { Color::new_no_nan_check(1.0, 1.0, 1.0, 1.0) }
    }
}
