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
use crate::{mutexes::Mutex, Color, Window};
use core::convert::TryInto;
use euclid::default::Point2D;
use flutterbug::{prelude::*, Color as FlColor, DisplayReference};
use hashbrown::HashMap;

// TODO: this naively assumes that only one display will be opened for the entire program
lazy_static::lazy_static! {
    static ref DPY_COLOR_MAPPING: Mutex<HashMap<Color, FlColor>> = Mutex::new(HashMap::new());
}

/// The graphics interface to a Flutterbug drawable object.
pub struct FlutterbugGraphics {
    window: Window,

    // we need a reference to the display for its colormap
    dpy: DisplayReference,
}

impl FlutterbugGraphics {
    #[inline]
    pub fn new(window: &Window) -> crate::Result<Self> {
        let dpy = window
            .inner_window()?
            .inner_flutter_window()
            .display_reference()
            .clone();

        Ok(Self {
            window: window.clone(),
            dpy,
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

// quick function to help convert a Color to the Beetle equivalent

// most methods can just be forwarded to the inner flutter window
impl InternalGraphics for FlutterbugGraphics {
    fn set_foreground(&self, clr: Color) -> crate::Result<()> {
        let inner = self.window.inner_window()?;
        inner
            .inner_flutter_window()
            .set_foreground(self.to_flcolor(clr)?)?;
        Ok(())
    }

    fn set_background(&self, clr: Color) -> crate::Result<()> {
        self.window
            .inner_window()?
            .inner_flutter_window()
            .set_foreground(self.to_flcolor(clr)?)?;
        Ok(())
    }

    fn draw_line(&self, p1: Point2D<u32>, p2: Point2D<u32>) -> crate::Result<()> {
        let x1: i32 = p1.x.try_into()?;
        let x2: i32 = p2.x.try_into()?;
        let y1: i32 = p1.y.try_into()?;
        let y2: i32 = p2.y.try_into()?;

        self.window
            .inner_window()?
            .inner_flutter_window()
            .draw_line(Point2D::new(x1, y1), Point2D::new(x2, y2))?;
        Ok(())
    }
}
