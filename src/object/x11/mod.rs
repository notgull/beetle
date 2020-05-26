/* -----------------------------------------------------------------------------------
 * src/object/x11/mod.rs - The root module for X11 peer objects.
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

use crate::{
    object::{GuiFactoryBase, PeerObject, WindowBase},
    Color, GenericWidget, GenericWidgetReference, Signal, Widget, SigDestroyWindow, SigDestroyApplication, SigCreated,
};
use euclid::default::{Point2D, Rect, Size2D};
use flutterbug::{
    prelude::*, x11::xlib, Color as FlutterColor, ColorMap, Context, Display, Event, Pixmap, Window,
};
use std::{collections::HashMap, convert::TryInto, sync::{Arc, RwLock}};

mod label;
pub use label::*;
mod window;
pub use window::*;

/// Some basic properties for a Beetle widget.
#[derive(Debug)]
pub(crate) struct BasicWidgetProperties {
    pub background: Color,
    pub foreground: Color,
    pub bounds: Rect<u32>,
    pub pixmap_cache: HashMap<u32, Pixmap>, // note: chars are cheaply convertible to u32
}

impl BasicWidgetProperties {
    pub fn new(bounds: Rect<u32>) -> Self {
        Self {
            bounds,
            background: Color::transparent(),
            foreground: Color::transparent(),
            pixmap_cache: HashMap::new(),
        }
    }
}

/// Process a Beetle color into an X11 color.
#[inline]
pub(crate) fn beetle_color_to_x11_color(bc: Color) -> FlutterColor {
    // macro to fit the float to the 0-u16::MAX range
    macro_rules! round_float {
        (bc.$n: ident) => {{
            (bc.$n() * (::std::u16::MAX as f32)).round() as u16
        }};
    }

    let fc = FlutterColor::Rgb {
        r: round_float!(bc.r),
        g: round_float!(bc.g),
        b: round_float!(bc.b),
    };

    fc
}

/// Process props for basic X11 events.
pub(crate) fn props_x11_event(
    xev: &Event,
    window: &Window,
    props: &mut BasicWidgetProperties,
) -> Result<(), crate::Error> {
    match &*xev {
        Event::Expose(e) => {
            // note: we can merge down the background color onto the default window white
            let background = props
                .background
                .merge_down(&unsafe { Color::with_alpha_unchecked(1.0, 1.0, 1.0, 1.0) })?;

            let sz = Size2D::new(e.width(), e.height());
            window.set_foreground(beetle_color_to_x11_color(background))?;
            window.fill_rectangle(Point2D::zero(), sz)?;
        }
        _ => {}
    }

    Ok(())
}

/// Default X11 event translator
pub(crate) fn default_x11_translate_event(xev: Event) -> Result<Vec<Arc<dyn Signal + 'static>>, crate::Error> {
    let mut evs: Vec<Arc<dyn Signal + 'static>> = Vec::with_capacity(1);
    match xev {
        Event::Map(_m) => evs.push(Arc::new(SigCreated)),
        _ => { /* do nothing */ } 
    }

    Ok(evs)
}

#[inline]
pub(crate) fn get_x_origin(rect: Rect<u32>) -> Point2D<i32> {
    euclid::point2(
        rect.origin.x.try_into().unwrap(),
        rect.origin.y.try_into().unwrap(),
    )
}

/// The X11 peer object factory. Contains the Display and the Context.
#[derive(Debug)]
pub struct X11GuiFactory {
    display: Display,
    context: RwLock<HashMap<xlib::Window, GenericWidgetReference>>,
}

impl X11GuiFactory {
    #[inline]
    pub(crate) fn display(&self) -> &Display {
        &self.display
    }
}

impl GuiFactoryBase for X11GuiFactory {
    type MainWindow = X11Window<X11MainWindow>;
    type ChildWindow = X11Window<X11ChildWindow>;
    type Label = X11Label;

    fn new() -> Result<Self, crate::Error> {
        Ok(Self {
            display: Display::new()?,
            context: RwLock::new(HashMap::new()),
        })
    }

    fn main_window(&self, bounds: Rect<u32>) -> Result<Self::MainWindow, crate::Error> {
        X11Window::new(self, (), bounds)
    }

    fn child_window(
        &self,
        parent: GenericWidgetReference,
        bounds: Rect<u32>,
    ) -> Result<Self::ChildWindow, crate::Error> {
        X11Window::new(self, parent, bounds)
    }

    fn label(
        &self,
        parent: GenericWidgetReference,
        bounds: Rect<u32>,
        text: String,
    ) -> Result<Self::Label, crate::Error> {
        X11Label::new(
            self,
            parent.inner_generic()?.internal_x11_window(),
            bounds,
            text,
        )
    }

    fn post_creation<T: PeerObject>(&self, widget: Widget<T>) -> Result<(), crate::Error> {
        self.context.try_write()?.insert(
            widget.inner()?.internal_x11_window().window(),
            widget.generic_reference(),
        );
        Ok(())
    }

    fn main_loop(self) -> Result<(), crate::Error> {
        // iterate over events
        'el: loop {
            let ev = Event::next(&self.display)?;
            let wid = ev.window();
            
            // get the widget that the event corresponds to
            if let Some(w) = self.context.try_read()?.get(&wid) {
                // translate the event
                let signals = w.inner_generic_mut()?.translate_x11_event(ev)?; 
 
                for signal in signals {
                    // TODO: send signal to signal handlers

                    // special signals need to be dealth with
                    if signal.is::<SigDestroyWindow>() {
                        self.context.try_write()?.remove(&wid);
                    } else if signal.is::<SigDestroyApplication>() {
                        break 'el;
                    }
                }
            }
        }

        Ok(())
    }
}
