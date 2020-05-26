/* -----------------------------------------------------------------------------------
 * src/object/x11/label.rs - The X11 label peer object.
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

use super::{BasicWidgetProperties, X11GuiFactory};
use crate::{
    object::{LabelBase, PeerObject, TextualBase},
    Font, Signal,
};
use euclid::default::{Point2D, Rect};
use flutterbug::{prelude::*, Color as FlutterColor, Event, EventMask, Window};
use std::{fmt, sync::Arc};

/// The label peer object.
#[derive(Debug)]
pub struct X11Label {
    props: BasicWidgetProperties,
    window: Window,
    text: String,
    font: Option<Font>,
    black: FlutterColor,
}

impl X11Label {
    pub(crate) fn new(
        factory: &X11GuiFactory,
        parent: &Window,
        bounds: Rect<u32>,
        text: String,
    ) -> Result<Self, crate::Error> {
        let window = factory.display().create_simple_window(
            Some(parent),
            super::get_x_origin(bounds),
            bounds.size,
            1,
            factory.display().default_white_pixel()?,
            factory.display().default_white_pixel()?,
        )?;

        window.map(false)?;
        window.select_input(EventMask::EXPOSURE_MASK)?;

        Ok(Self {
            props: BasicWidgetProperties::new(bounds),
            window,
            text,
            font: None,
            black: factory.display().default_black_pixel()?,
        })
    }
}

impl PeerObject for X11Label {
    fn set_bounds(&mut self, bounds: Rect<u32>) -> Result<(), crate::Error> {
        Ok(self
            .window
            .set_bounds(super::get_x_origin(bounds), bounds.size)?)
    }

    fn set_parent(&mut self, _parent: &dyn PeerObject) -> Result<(), crate::Error> {
        unimplemented!()
    }

    fn internal_x11_window(&self) -> &Window {
        &self.window
    }

    fn translate_x11_event(&mut self, xev: Event) -> Result<Vec<Arc<dyn Signal + 'static>>, crate::Error> {
        super::props_x11_event(&xev, &self.window, &mut self.props)?;
        // TODO: fonts
        // TODO: newline
        if let &Event::Expose(ref _e) = &xev {
            self.window.set_foreground(self.black)?;
            self.window.draw_string(Point2D::new(0,10), &self.text)?;
        }
        super::default_x11_translate_event(xev)
    }
}

impl TextualBase for X11Label {
    fn set_text(&mut self, txt: String) -> Result<(), crate::Error> {
        self.text = txt;
        // TODO: set refresh
        Ok(())
    }

    fn set_font(&mut self, font: &Font) -> Result<(), crate::Error> {
        self.font = Some(font.clone());
        // TODO: refresh
        Ok(())
    }
}

impl LabelBase for X11Label {}
