/* -----------------------------------------------------------------------------------
 * src/arc.rs - Storage strategy for Arcs.
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

use euclid::{default::Rect, Angle};
use ordered_float::NotNan;

/// A geometric arc. This is called a "GeometricArc" to differentiate it from the
/// standard library type "Arc".
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GeometricArc {
    bounding_rect: Rect<u32>,
    start_angle: Angle<NotNan<f32>>,
    end_angle: Angle<NotNan<f32>>,
}

impl GeometricArc {
    /// Create a new geometric arc. This will return an error if the angles are NaN.
    #[inline]
    pub fn new(bounds: Rect<u32>, start: Angle<f32>, end: Angle<f32>) -> crate::Result<Self> {
        Ok(Self {
            bounding_rect: bounds,
            start_angle: Angle::radians(NotNan::new(start.radians)?),
            end_angle: Angle::radians(NotNan::new(end.radians)?),
        })
    }

    /// Create a new geometric arc without checking if the angles are NaN.
    #[inline]
    pub unsafe fn new_unchecked(bounds: Rect<u32>, start: Angle<f32>, end: Angle<f32>) -> Self {
        Self {
            bounding_rect: bounds,
            start_angle: Angle::radians(NotNan::unchecked_new(start.radians)),
            end_angle: Angle::radians(NotNan::unchecked_new(end.radians)),
        }
    }

    /// The bounding rectangle that contains the ellipse of the Arc.
    #[inline]
    pub fn bounds(&self) -> Rect<u32> {
        self.bounding_rect
    }

    /// Set the bounding rectangle. This rectangle defines the ellipse that the Arc is.
    /// a part of.
    #[inline]
    pub fn set_bounds(&mut self, bnds: Rect<u32>) {
        self.bounding_rect = bnds;
    }

    /// The angle in the ellipse where the arc starts.
    #[inline]
    pub fn start_angle(&self) -> Angle<f32> {
        Angle::radians(self.start_angle.radians.into_inner())
    }

    /// Set the angle in the ellipse where the arc starts. This will generate an
    /// error if the angle is NaN.
    #[inline]
    pub fn set_start_angle(&mut self, angle: Angle<f32>) -> crate::Result<()> {
        self.start_angle = Angle::radians(NotNan::new(angle.radians)?);
        Ok(())
    }

    /// Set the angle in the ellipse where the arc starts. This does not check for NaN.
    #[inline]
    pub unsafe fn set_start_angle_unchecked(&mut self, angle: Angle<f32>) {
        self.start_angle = Angle::radians(NotNan::unchecked_new(angle.radians));
    }

    /// The angle in the ellipse where the arc ends.
    #[inline]
    pub fn end_angle(&self) -> Angle<f32> {
        Angle::radians(self.end_angle.radians.into_inner())
    }

    /// Set the angle in the ellipse where the arc ends. This will generate an error
    /// if the angle is NaN.
    #[inline]
    pub fn set_end_angle(&mut self, angle: Angle<f32>) -> crate::Result<()> {
        self.end_angle = Angle::radians(NotNan::new(angle.radians)?);
        Ok(())
    }

    /// Set the angle in the ellipse where the arc ends. This does not check for NaN.
    #[inline]
    pub unsafe fn set_end_angle_unchecked(&mut self, angle: Angle<f32>) {
        self.end_angle = Angle::radians(NotNan::unchecked_new(angle.radians));
    }
}
