/* -----------------------------------------------------------------------------------
 * src/graphics/flutter.rs - Flutterbug graphics object.
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
use crate::{mutexes::Mutex, Color, GeometricArc, Window};
use alloc::sync::Arc;
use core::convert::TryInto;
use euclid::default::{Point2D, Rect};
use flutterbug::{prelude::*, Color as FlColor, DisplayReference};
use hashbrown::HashMap;

// TODO: this naively assumes that only one display will be opened for the entire program
lazy_static::lazy_static! {
    static ref DPY_COLOR_MAPPING: Mutex<HashMap<Color, FlColor>> = Mutex::new(HashMap::new());
}

struct ColorInfo {
    background: Option<FlColor>,
    foreground: FlColor,
}

/// The graphics interface to a Flutterbug drawable object.
pub struct FlutterbugGraphics {
    window: Window,

    // we need a reference to the display for its colormap
    dpy: DisplayReference,

    // see below for why we need this
    color_info: Arc<Mutex<ColorInfo>>,
}

impl Clone for FlutterbugGraphics {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            window: self.window.clone(),
            dpy: self.dpy.clone(),
            color_info: self.color_info.clone(),
        }
    }
}

impl FlutterbugGraphics {
    #[inline]
    pub fn new(window: &Window) -> crate::Result<Self> {
        let inner_flutter = window.fl_inner_window().unwrap();
        let dpy = inner_flutter.display_reference().clone();

        // set the defaults
        let black = dpy.default_black_pixel()?;
        let white = dpy.default_white_pixel()?;
        inner_flutter.set_foreground(black)?;
        inner_flutter.set_background(white)?;

        Ok(Self {
            window: window.clone(),
            dpy,
            color_info: Arc::new(Mutex::new(ColorInfo {
                foreground: black,
                background: None,
            })),
        })
    }

    // helper function to convert a Beetle color to a Flutterbug color
    #[inline]
    fn to_flcolor(&self, color: Color) -> crate::Result<FlColor> {
        let mut clr_mapping = DPY_COLOR_MAPPING.lock();

        // if it's not already in the color mapping, run it through the colormap
        // of the display
        match clr_mapping.get(&color) {
            Some(fl) => Ok(*fl),
            None => {
                // convert a float to a ushort
                macro_rules! f2us {
                    ($a: expr) => {{
                        // TODO: not naively assume the c_ushort = u16
                        ($a * (core::u16::MAX as f32)) as u16
                    }};
                }

                let clrmap = self.dpy.default_colormap()?;
                clr_mapping.insert(
                    color,
                    clrmap.color(FlColor::Rgb {
                        r: f2us!(color.r()),
                        g: f2us!(color.g()),
                        b: f2us!(color.b()), // TODO; account for alpha
                    })?,
                );
                Ok(*clr_mapping.get(&color).unwrap())
            }
        }
    }
}

// most methods can just be forwarded to the inner flutter window
impl InternalGraphics for FlutterbugGraphics {
    #[inline]
    fn set_foreground(&self, clr: Color) -> crate::Result<()> {
        let clr = self.to_flcolor(clr)?;

        self.window.fl_inner_window().unwrap().set_foreground(clr)?;
        self.color_info.lock().foreground = clr;
        Ok(())
    }

    #[inline]
    fn set_background(&self, clr: Color) -> crate::Result<()> {
        let clr = self.to_flcolor(clr)?;
        self.window.fl_inner_window().unwrap().set_background(clr)?;
        self.color_info.lock().background = Some(clr);
        Ok(())
    }

    #[inline]
    fn set_line_width(&self, lw: u32) -> crate::Result<()> {
        self.window.fl_inner_window().unwrap().set_line_width(lw)?;
        Ok(())
    }

    #[inline]
    fn draw_line(&self, p1: Point2D<u32>, p2: Point2D<u32>) -> crate::Result<()> {
        let x1: i32 = p1.x.try_into()?;
        let x2: i32 = p2.x.try_into()?;
        let y1: i32 = p1.y.try_into()?;
        let y2: i32 = p2.y.try_into()?;

        self.window
            .fl_inner_window()
            .unwrap()
            .draw_line(Point2D::new(x1, y1), Point2D::new(x2, y2))?;
        Ok(())
    }

    #[inline]
    fn draw_rectangle(&self, rect: Rect<u32>) -> crate::Result<()> {
        let origin = Point2D::<i32>::new(rect.origin.x.try_into()?, rect.origin.y.try_into()?);
        let size = rect.size;

        let ifl = self.window.fl_inner_window().unwrap();
        ifl.draw_rectangle(origin, size)?;

        // temporarily switch the background color to the foreground so we
        // can use it to fill
        let clock = self.color_info.lock();
        if let Some(clr) = clock.background {
            ifl.set_foreground(clr)?;
            ifl.fill_rectangle(origin, size)?;
            ifl.set_foreground(clock.foreground)?;
        }

        Ok(())
    }

    #[inline]
    fn draw_arc(&self, arc: GeometricArc) -> crate::Result<()> {
        // X11 protocol mandates that we use angles multiplied by 64
        macro_rules! to_x11_angle {
            ($ang: expr) => {{
                ($ang.radians * 64.0) as i32
            }};
        }

        let angles = (
            to_x11_angle!(arc.start_angle()),
            to_x11_angle!(arc.end_angle()),
        );
        let bounds = arc.bounds();
        let origin = Point2D::<i32>::new(bounds.origin.x.try_into()?, bounds.origin.y.try_into()?);
        let size = bounds.size;

        let ifl = self.window.fl_inner_window().unwrap();
        ifl.draw_arc(origin, size, angles)?;

        // fill if we need to
        let clock = self.color_info.lock();
        if let Some(clr) = clock.background {
            ifl.set_foreground(clr)?;
            ifl.fill_arc(origin, size, angles)?;
            ifl.set_foreground(clock.foreground)?;
        }

        Ok(())
    }
}
