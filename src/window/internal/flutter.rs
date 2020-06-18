/* -----------------------------------------------------------------------------------
 * src/window/internal/flutter.rs - Flutterbug-oriented internal window.
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

use super::{super::Window, unique_id, EventHandler, GenericWindowInternal};
use crate::{Instance, Texture};
use euclid::default::{Point2D, Rect};
use flutterbug::{prelude::*, InputContext, Window as FWindow};
use std::{boxed::Box, mem};

pub struct WindowInternal {
    inner: FWindow,
    id: usize,
    event_handler: Box<dyn EventHandler>,
    text: String,
    background: Option<Texture>,
    ic: InputContext,
}

impl GenericWindowInternal for WindowInternal {
    fn id(&self) -> usize {
        self.id
    }

    fn new(
        instance: &Instance,
        parent: Option<&Window>,
        _class_name: String,
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
    ) -> crate::Result<Self> {
        // create the struct representing the internal flutterbug window
        let dpy = instance.display();
        let inner = dpy.create_simple_window(
            parent.map(|p| p.inner_flutter_window()).as_deref(),
            Point2D::new(bounds.origin.x as i32, bounds.origin.y as i32),
            bounds.size,
            1,
            dpy.default_white_pixel()?,
            dpy.default_white_pixel()?,
        )?;
        Ok(WindowInternal {
            id: unique_id(),
            event_handler: Box::new(super::default_event_handler),
            text,
            background,
            ic: inner.input_context(instance.im())?,
            inner,
        })
    }

    fn event_handler(&self) -> &dyn EventHandler {
        &*self.event_handler
    }

    fn set_event_handler<F: EventHandler>(&mut self, evh: F) {
        self.event_handler = Box::new(evh);
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, txt: String) -> String {
        let mut res = txt;
        mem::swap(&mut self.text, &mut res);
        res
    }
}

impl WindowInternal {
    /// Get the internal Flutterbug window.
    pub fn inner_flutter_window(&self) -> &FWindow {
        &self.inner
    }

    /// Get the input context.
    pub fn ic(&self) -> &InputContext {
        &self.ic
    }
}
