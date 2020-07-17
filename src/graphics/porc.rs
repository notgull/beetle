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
use crate::{colors, mutexes::Mutex, Color, GeometricArc, Window};
use alloc::sync::Arc;
use core::convert::TryInto;
use cty::c_int;
use euclid::default::{Point2D, Rect};
use porcupine::{prelude::*, Brush, DeviceContext, Pen, PenStyle};

// the pen/brush info needs to be stored along with the graphics context
struct PenAndBrush {
    pen: Option<Pen>,
    // also store the current line width and color
    line_width: u32,
    pen_color: Color,

    brush: Option<Brush>,
    // also store the current color
    brush_color: Color,
}

struct PorcupineGraphicsInner {
    dc: DeviceContext,
    pb: Mutex<PenAndBrush>,
}

#[derive(Clone)]
#[repr(transparent)]
pub struct PorcupineGraphics(Arc<PorcupineGraphicsInner>);

impl PorcupineGraphics {
    #[inline]
    pub fn new(wnd: &Window) -> crate::Result<Self> {
        let dc = wnd.prc_inner_window().unwrap().begin_paint()?;
        dc.set_pen_color(0, 0, 0)?;
        dc.set_brush_color(core::u8::MAX, core::u8::MAX, core::u8::MAX)?;

        Ok(Self(Arc::new(PorcupineGraphicsInner {
            dc,
            pb: Mutex::new(PenAndBrush {
                pen: None,
                brush: None,
                line_width: 1,
                pen_color: colors::black(),
                brush_color: colors::white(),
            }),
        })))
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

#[inline]
fn cnvrt_rect(rect: Rect<u32>) -> crate::Result<Rect<c_int>> {
    Ok(euclid::rect(
        rect.origin.x.try_into()?,
        rect.origin.y.try_into()?,
        rect.size.width.try_into()?,
        rect.size.height.try_into()?,
    ))
}

impl InternalGraphics for PorcupineGraphics {
    fn set_foreground(&self, clr: Color) -> crate::Result<()> {
        let (r, g, b) = clip_color(clr);
        // create a new pen from the color and stored line_width
        let mut pb = self.0.pb.lock();

        pb.pen_color = clr;
        let pen = Pen::new(r, g, b, pb.line_width, PenStyle::Solid)?;
        self.0.dc.set_pen(&pen);
        pb.pen = Some(pen);

        Ok(())
    }

    fn set_background(&self, clr: Color) -> crate::Result<()> {
        let (r, g, b) = clip_color(clr);
        self.0.dc.set_brush_color(r, g, b)?;
        Ok(())
    }

    fn set_line_width(&self, lw: u32) -> crate::Result<()> {
        // create a new pen from the stored color
        let mut pb = self.0.pb.lock();
        pb.line_width = lw;

        let (r, g, b) = clip_color(pb.pen_color);
        let pen = Pen::new(r, g, b, lw, PenStyle::Solid)?;
        self.0.dc.set_pen(&pen);
        pb.pen = Some(pen);

        Ok(())
    }

    fn draw_line(&self, p1: Point2D<u32>, p2: Point2D<u32>) -> crate::Result<()> {
        let x1: c_int = p1.x.try_into()?;
        let x2: c_int = p2.x.try_into()?;
        let y1: c_int = p1.y.try_into()?;
        let y2: c_int = p2.y.try_into()?;

        self.0
            .dc
            .draw_line(Point2D::new(x1, y1), Point2D::new(x2, y2))?;
        Ok(())
    }

    fn draw_rectangle(&self, rect: Rect<u32>) -> crate::Result<()> {
        let rect: Rect<c_int> = cnvrt_rect(rect)?;
        self.0.dc.draw_rect(rect)?;
        Ok(())
    }

    fn draw_arc(&self, arc: GeometricArc) -> crate::Result<()> {
        let bounds = cnvrt_rect(arc.bounds())?;
        unimplemented!()
    }

    fn draw_ellipse(&self, bounds: Rect<u32>) -> crate::Result<()> {
        let bounds = cnvrt_rect(bounds)?;
        self.0.dc.draw_ellipse(bounds)?;
        Ok(())
    }
}
