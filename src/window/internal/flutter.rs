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
use crate::{EventType, Instance, Texture};
use alloc::{boxed::Box, string::String, sync::Arc};
use core::{convert::TryInto, mem};
use euclid::default::{Point2D, Rect};
use flutterbug::{
    prelude::*, Event as FEvent, EventMask, EventType as FEventType, ExposeEvent, InputContext,
    Window as FWindow,
};
use hashbrown::{HashMap, HashSet};
use smallvec::{smallvec, SmallVec};

pub struct WindowInternal {
    inner: FWindow,
    id: usize,
    event_handler: Box<dyn EventHandler>,
    text: String,
    background: Option<Texture>,
    ic: InputContext,
    top_level: bool,
    bounds: Rect<u32>,
}

impl GenericWindowInternal for WindowInternal {
    #[inline]
    fn id(&self) -> usize {
        self.id
    }

    fn new(
        instance: &Instance,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
        top_level: bool,
    ) -> crate::Result<Self> {
        // create the struct representing the internal flutterbug window
        let dpy = instance.display();
        // TODO: let's not duplicate code here
        let inner = match parent {
            Some(ref p) => {
                let mut p2 = p.inner_window();

                dpy.create_simple_window(
                    Some(p2.inner_flutter_window()),
                    Point2D::new(bounds.origin.x.try_into()?, bounds.origin.y.try_into()?),
                    bounds.size,
                    1,
                    dpy.default_white_pixel()?,
                    dpy.default_white_pixel()?,
                )
            }
            None => dpy.create_simple_window(
                None,
                Point2D::new(bounds.origin.x.try_into()?, bounds.origin.y.try_into()?),
                bounds.size,
                1,
                dpy.default_white_pixel()?,
                dpy.default_white_pixel()?,
            ),
        }?;

        inner.set_protocols(&mut [instance.delete_window_atom()])?;
        inner.store_name(&text)?;
        inner.select_input(EventMask::EXPOSURE_MASK)?;

        Ok(WindowInternal {
            id: unique_id(),
            event_handler: Box::new(super::default_event_handler),
            bounds,
            text,
            background,
            ic: inner.input_context(instance.im())?,
            inner,
            top_level,
        })
    }

    #[inline]
    fn event_handler(&self) -> &dyn EventHandler {
        &*self.event_handler
    }

    #[inline]
    fn set_event_handler<F: EventHandler>(&mut self, evh: F) {
        self.event_handler = Box::new(evh);
    }

    #[inline]
    fn text(&mut self) -> &mut str {
        &mut self.text
    }

    #[inline]
    fn set_text(&mut self, txt: String) -> crate::Result<String> {
        self.inner.store_name(&txt)?;

        let mut res = txt;
        mem::swap(&mut self.text, &mut res);
        Ok(res)
    }

    #[inline]
    fn is_top_level(&self) -> bool {
        self.top_level
    }

    #[inline]
    fn bounds(&self) -> Rect<u32> {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect<u32>, backend: bool) -> crate::Result<Rect<u32>> {
        if backend {
            self.inner.set_bounds(
                Point2D::new(bounds.origin.x.try_into()?, bounds.origin.y.try_into()?),
                bounds.size,
            )?;
        }

        let mut res = bounds;
        mem::swap(&mut self.bounds, &mut res);
        Ok(res)
    }

    #[inline]
    fn show(&self) -> crate::Result<()> {
        Ok(self.inner.map(true)?)
    }

    fn receive_events(&mut self, events: &[EventType]) -> crate::Result<()> {
        // figure out which events correspond to which X11 event masks
        lazy_static::lazy_static! {
            static ref X11_EVENT_MAPPING: HashMap<EventType, SmallVec<[EventMask; 1]>> = {
                let mut map = HashMap::new();
                map.insert(EventType::KeyDown, smallvec![EventMask::KEY_PRESS_MASK]);
                map.insert(EventType::KeyUp, smallvec![EventMask::KEY_RELEASE_MASK]);

                map.insert(EventType::MouseButtonDown, smallvec![EventMask::BUTTON_PRESS_MASK]);
                map.insert(EventType::MouseButtonUp, smallvec![EventMask::BUTTON_RELEASE_MASK]);

                // TODO: add more events
                map
            };
        }

        // insert the corresponding event masks into the event set
        let event_set = events
            .iter()
            .map(|et| X11_EVENT_MAPPING.get(et))
            .filter(Option::is_some)
            .flat_map(|fetl| fetl.unwrap().iter())
            .copied()
            .collect::<HashSet<EventMask>>();

        // exit early if the event set is empty
        if event_set.is_empty() {
            return Ok(()); // TODO: maybe undefined behavior if this function is called more than once?
        }

        let mut sum_event_mask = EventMask::EXPOSURE_MASK; // EXPOSURE_MASK is there no matter what
        for e in event_set {
            sum_event_mask |= e;
        }

        Ok(self.inner.select_input(sum_event_mask)?)
    }

    fn repaint(&self, bounds: Option<Rect<u32>>) -> crate::Result<()> {
        let bounds = bounds.or_else(|| Some(self.bounds())).unwrap(); // shouldn't fail

        let ev = ExposeEvent::new(
            FEventType::Expose,
            0,
            self.inner.display_reference(),
            &self.inner,
            true,
            bounds.origin.x.try_into()?,
            bounds.origin.y.try_into()?,
            bounds.size.width,
            bounds.size.height,
            1,
        )?;
        let ev = FEvent::Expose(ev);
        Ok(ev.send(
            self.inner.display_reference(),
            &self.inner,
            true,
            EventMask::EXPOSURE_MASK,
        )?)
    }

    fn background(&mut self) -> Option<&mut Texture> {
        self.background.as_mut()
    }

    fn set_background(&mut self, texture: Option<Texture>) {
        self.background = texture;
        // should send a repaint event on the top level
    }

    fn take_background(&mut self) -> Option<Texture> {
        self.background.take()
    }
}

impl WindowInternal {
    /// Get the internal Flutterbug window.
    pub fn inner_flutter_window(&mut self) -> &mut FWindow {
        &mut self.inner
    }

    /// Get the input context.
    pub fn ic(&mut self) -> &mut InputContext {
        &mut self.ic
    }
}
