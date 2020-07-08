/* -----------------------------------------------------------------------------------
 * src/graphics/porc.rs - Graphics framework that hooks up to Porcupine.
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

use super::InternalGraphics;
use crate::{Color, Window};
use core::convert::TryInto;
use cty::c_int;
use euclid::default::Point2D;
use porcupine::{prelude::*, DeviceContext};

#[repr(transparent)]
pub struct PorcupineGraphics(DeviceContext);

impl PorcupineGraphics {
    #[inline]
    pub fn new(wnd: &Window) -> crate::Result<Self> {
        Ok(Self(wnd.inner_window()?.inner_porc_window().begin_paint()?))
    }
}

// drop should automatically end the paint

// clip a color to the 256 range
// TODO: account for alpha
#[inline]
fn clip_color(clr: Color) -> (u8, u8, u8) {
    macro_rules! cnvrt {
        ($val: expr) => {{
            ($val * (core::u8::MAX as f32)) as u8
        }};
    }

    (cnvrt!(clr.r()), cnvrt!(clr.g()), cnvrt!(clr.b()))
}

impl InternalGraphics for PorcupineGraphics {
    fn set_foreground(&self, clr: Color) -> crate::Result<()> {
        let (r, g, b) = clip_color(clr);
        self.0.set_pen_color(r, g, b)?;
        Ok(())
    }

    fn set_background(&self, clr: Color) -> crate::Result<()> {
        let (r, g, b) = clip_color(clr);
        self.0.set_brush_color(r, g, b)?;
        Ok(())
    }

    fn draw_line(&self, p1: Point2D<u32>, p2: Point2D<u32>) -> crate::Result<()> {
        let x1: c_int = p1.x.try_into()?;
        let x2: c_int = p2.x.try_into()?;
        let y1: c_int = p1.y.try_into()?;
        let y2: c_int = p2.y.try_into()?;

        self.0
            .draw_line(Point2D::new(x1, y1), Point2D::new(x2, y2))?;
        Ok(())
    }
}
