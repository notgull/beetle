/* -----------------------------------------------------------------------------------
 * src/window/mod.rs - Define the Window.
 * beetle - Pull-based GUI framework
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

mod id;
pub(crate) mod internal;
#[cfg(feature = "expose_internal")]
pub use internal::*;
#[cfg(not(feature = "expose_internal"))]
pub(crate) use internal::*;

use crate::{
    mutexes::{Mutex, RwLock},
    Event, EventData, EventType, Instance, InstanceType, Pixel, Texture,
};
use alloc::{string::String, sync::Arc};
use core::{
    fmt,
    hash::{Hash, Hasher},
    mem,
};
use euclid::{Point2D, Rect, Size2D};
use hashbrown::HashSet;

/// Miscellaneous properties a window can hold.
pub(crate) struct WindowProperties {
    text: String,
    bounds: Rect<u32, Pixel>,
    background: Option<Texture>,
    is_top_level: bool,
}

impl WindowProperties {
    pub fn new(
        text: String,
        bounds: Rect<u32, Pixel>,
        background: Option<Texture>,
        is_top_level: bool,
    ) -> Self {
        Self {
            text,
            bounds,
            background,
            is_top_level,
        }
    }
}

struct WindowInner {
    // the backend window
    backend: internal::InternalWindow,
    // the properties held by the window. write access is needed a lot, so a Mutex is used.
    properties: Mutex<WindowProperties>,
    // due to the ID's copy status, we keep it in the inner window
    id: usize,
    // the events received by the window this is read more often than it's written to,
    // hence the RwLock
    events_received: RwLock<HashSet<EventType>>,
}

/// An abstraction over the window. This is a rectangle of windows on the screen.
pub struct Window(
    Arc<WindowInner>,
    // note: a reference to the instance is held here, so I can clone it with the window
    Instance,
);

impl Clone for Window {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl fmt::Debug for Window {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Window")
            .field("id", &self.0.id)
            .field("instance", &self.1)
            .finish()
    }
}

impl Hash for Window {
    #[inline]
    fn hash<H: Hasher>(&self, h: &mut H) {
        Hash::hash(&self.0.id, h)
    }
}

impl PartialEq for Window {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

impl Eq for Window {}

impl Window {
    /// Create a new window from the raw parts.
    pub(crate) fn from_raw(
        backend: internal::InternalWindow,
        properties: Mutex<WindowProperties>,
        instance: Instance,
    ) -> Self {
        Self(
            Arc::new(WindowInner {
                backend,
                properties,
                events_received: RwLock::new(HashSet::new()),
                id: id::unique_id(),
            }),
            instance,
        )
    }

    /// Get the ID for this widget.
    #[inline]
    pub fn id(&self) -> usize {
        self.0.id
    }

    /// Get whether this is a top-level window.
    #[inline]
    pub fn is_top_level(&self) -> bool {
        self.0.properties.lock().is_top_level
    }

    /// Get the current bounds of this window.
    #[inline]
    pub fn bounds(&self) -> Rect<u32, Pixel> {
        self.0.properties.lock().bounds
    }

    /// Get the current size of this window.
    #[inline]
    pub fn size(&self) -> Size2D<u32, Pixel> {
        self.bounds().size
    }

    /// Set the size, and choose whether or not to emit a backend message.
    #[inline]
    pub(crate) fn set_size_emit_choice(&self, bounds: Size2D<u32, Pixel>) -> crate::Result<()> {
        match (self.is_top_level(), self.1.ty()) {
            (true, InstanceType::Flutterbug) => {
                let mut old_bounds = bounds;
                let new_bounds = bounds;
                mem::swap(&mut self.0.properties.lock().bounds.size, &mut old_bounds);

                // we need to handle resizing events manually
                // just produce a Resizing event
                let mut rev = Event::new(
                    self,
                    EventData::Resizing {
                        old: old_bounds,
                        new: Arc::new(RwLock::new(new_bounds)),
                    },
                );
                // the hidden data bool indicates whether we should release a SizeChanged event
                // since we are handling it manually, it is true
                rev.set_hidden_data(true);
                self.1.queue_event(rev);
                Ok(())
            }
            (_, _) => {
                // otherwise, just resize the window on the backend
                // this emits the signal that sets the size
                self.backend().set_size(bounds)
            }
        }
    }

    /// Set the size of this window.
    #[inline]
    pub fn set_size(&self, bounds: Size2D<u32, Pixel>) -> crate::Result<()> {
        self.set_size_emit_choice(bounds)
    }

    // helper function to get the generic backend
    #[inline]
    fn backend(&self) -> &dyn GenericInternalWindow {
        self.0.backend.generic()
    }

    /// Tell which events to receive.
    #[inline]
    pub fn receive_events(&self, events: &[EventType]) -> crate::Result<()> {
        *(self
            .0
            .events_received
            .try_write()
            .ok_or_else(|| crate::Error::UnableToWrite)?) = events.iter().copied().collect();
        Ok(())
    }

    /// Receive an event.
    #[inline]
    pub fn handle_event(&self, event: &mut Event) -> crate::Result<()> {
        let hidden_data = event.take_hidden_data();
        let data = event.data().clone();
        match data {
            EventData::Resizing { old, new } => {
                if let Some(t) = hidden_data.and_then(|hd| Arc::downcast::<bool>(hd).ok()) {
                    if *t {
                        // if the new size is different than the one we sent with,
                        // set it as so
                        let new = new.read().clone();
                        if new != self.size() {
                            self.0.properties.lock().bounds.size = new;
                        }

                        self.backend().set_size(new)?; // shouldn't be recursive

                        self.1
                            .queue_event(Event::new(self, EventData::Resized { old, new }));
                    }
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Some handling for events needs to happen prior to dispatch.
    #[inline]
    pub(crate) fn handle_event_before_dispatch(&self, event: &Event) -> crate::Result<()> {
        match event.data() {
            EventData::Resized { old: _, new } => {
                // if we aren't handling it ourselves, set the size manually
                if let (InstanceType::Flutterbug, true) = (self.1.ty(), self.is_top_level()) {
                    /* do nothing */
                } else {
                    self.0.properties.lock().bounds.size = *new;
                }
            }
            _ => (),
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
impl Window {
    pub(crate) fn fl_inner_window(&self) -> Option<&flutterbug::Window> {
        match self.0.backend {
            InternalWindow::Flutter(ref f) => Some(f.fl_window()),
            _ => None,
        }
    }

    pub(crate) fn fl_input_context(&self) -> Option<&flutterbug::InputContext> {
        match self.0.backend {
            InternalWindow::Flutter(ref f) => Some(f.ic()),
            _ => None,
        }
    }
}

#[cfg(windows)]
impl Window {
    pub(crate) fn prc_inner_window(&self) -> Option<&porcupine::Window> {
        match self.0.backend {
            InternalWindow::Porcupine(ref p) => Some(p.prc_window()),
            _ => None,
        }
    }
}
