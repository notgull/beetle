/* -----------------------------------------------------------------------------------
 * src/object/linux/x11/label.rs - This should define a subobject for the X11 label.
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

use super::gui_object::{GuiObject, GuiTextual, LabelBase};
use crate::Font;
use nalgebra::geometry::Point4;
use std::{
    os::raw::c_int,
    ptr::{self, NonNull},
};
use x11::xlib::{self, Display, Window};

#[derive(Debug)]
pub struct X11Label {
    bounds: Point4<u32>,
    text: String,
    //display: DisplayPointer,
    //window: Window,
}

impl X11Label {
    pub fn new(bounds: Point4<u32>, text: &str, _font: Option<&Font>) -> Self {
        Self {
            bounds,
            text: text.to_string(),
        }
    }
}

impl GuiTextual for X11Label {
    fn set_text(&mut self, val: &str) -> Result<(), crate::Error> {
        self.text = val.to_string();
        Ok(())
    }
}

impl LabelBase for X11Label {}

impl GuiObject for X11Label {
    fn bounds(&self) -> Point4<u32> {
        self.bounds
    }
    fn set_bounds(&mut self, bounds: Point4<u32>) -> Result<(), crate::Error> {
        self.bounds = bounds;
        // TODO: for now, let's assume that the upper widget will take care of re-rendering for us
        //x11_utils::force_x11_redraw(do_upgrade(self.display)?.as_ptr(), self.window, bounds)
        Ok(())
    }
    fn set_parent(&mut self, parent: &dyn GuiObject) -> Result<(), crate::Error> {
        if let Some(w) = parent.get_x11_window() {
            //self.window = w;
        } // container cases should be handled by real objects
        Ok(())
    }
    #[inline]
    fn get_x11_window(&self) -> Option<Window> {
        None
    }
    #[inline]
    fn get_x11_gc(&self) -> Option<ptr::NonNull<xlib::_XGC>> {
        None
    }

    fn render(
        &self,
        display: &ptr::NonNull<Display>,
        win: Window,
        gc: NonNull<xlib::_XGC>,
    ) -> Result<(), crate::Error> {
        let display = display.as_ptr();
        // TODO: use font bitmaps instead of XDrawString
        unsafe {
            xlib::XDrawString(
                display,
                win,
                gc.as_ptr(),
                self.bounds.x as c_int,
                self.bounds.y as c_int,
                crate::utils::to_cstring(&self.text)?,
                self.text.bytes().count() as c_int,
            )
        };
        Ok(())
    }
}
