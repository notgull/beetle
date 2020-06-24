/* -----------------------------------------------------------------------------------
 * src/window/mod.rs - Define the Window structure.
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

use crate::{Event, EventType, Instance, ReadOnlyMappedMutexGuard, Texture};
use euclid::default::Rect;
use parking_lot::Mutex;
use std::{
    any::Any,
    collections::HashSet,
    fmt,
    hash::{Hash, Hasher},
    sync::Arc,
};

mod id;
pub(crate) use id::*;

pub mod internal;
pub use internal::EventHandler;
pub use internal::*;

lazy_static::lazy_static! {
    // default events allowed
    static ref DEFAULT_EVENTS: [EventType; 10] = [
        EventType::AboutToPaint,
        EventType::Paint,
        EventType::TextChanging,
        EventType::TextChanged,
        EventType::Quit,
        EventType::Close,
        EventType::BoundsChanging,
        EventType::BoundsChanged,
        EventType::BackgroundChanging,
        EventType::BackgroundChanged
    ];
}

/// A rectangle of pixels on the screen, in the most basic terms. This structure is actually
/// a cheaply copyable wrapper around the internal window object.
///
/// It is of note that Beetle uses the term "Window" differently than how other graphics
/// frameworks use it. In the most basic terms, a Window is a rectangle of pixels that
/// appear on the screen. The frame containing all of your widgets is a Window. The widgets
/// within that frame are all considered Windows. In some cases, you can even consider
/// the screen itself a Window.
///
/// The Window object is created using the Instance object.
pub struct Window {
    inner: Arc<Mutex<WindowInternal>>,
    handled_events: Arc<Mutex<HashSet<EventType>>>,
    instance: Instance,
    id: usize,

    // make sure it owns any extra data
    _extra_data: Option<Arc<dyn Any>>,
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Beetle Window #{}", self.id(),)
    }
}

impl Hash for Window {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.id().hash(h);
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }

    fn ne(&self, other: &Self) -> bool {
        self.id() != other.id()
    }
}

impl Eq for Window {}

impl Clone for Window {
    fn clone(&self) -> Self {
        Self::from_raw(
            self.inner.clone(),
            self.handled_events.clone(),
            self.id,
            self.instance.clone(),
            self._extra_data.clone(),
        )
    }
}

impl Window {
    /// Internal function to create a new Window.
    #[inline]
    pub(crate) fn from_raw(
        inner: Arc<Mutex<WindowInternal>>,
        handled_events: Arc<Mutex<HashSet<EventType>>>,
        id: usize,
        instance: Instance,
        extra_data: Option<Arc<dyn Any>>,
    ) -> Self {
        Self {
            inner,
            handled_events,
            id,
            instance,
            _extra_data: extra_data,
        }
    }

    /// Get the text associated with this window.
    #[inline]
    pub fn text(&self) -> ReadOnlyMappedMutexGuard<'_, str> {
        ReadOnlyMappedMutexGuard::from_guard(self.inner.lock(), |i| i.text())
    }

    /// Set the text associated with this window. This will emit a TextChanged event.
    #[inline]
    pub fn set_text(&self, text: String) -> crate::Result<()> {
        let mut l = self.inner.lock();
        self.instance.queue_event(Event::new(
            self,
            EventType::TextChanging,
            vec![Arc::new(l.text().to_string()), Arc::new(text)],
        ));
        Ok(())
    }

    fn set_text_internal(&self, text: String) -> crate::Result<()> {
        let mut l = self.inner.lock();
        let cloned_text = Arc::new(text.clone());
        self.instance.queue_event(Event::new(
            self,
            EventType::TextChanged,
            vec![Arc::new(l.set_text(text)?), cloned_text],
        ));
        Ok(())
    }

    /// Set the event handler. This will not emit an event.
    ///
    /// The Event Handler is a function run after normal event processing is done.
    #[inline]
    pub fn set_event_handler<F: EventHandler>(&self, evh: F) {
        let mut l = self.inner.lock();
        l.set_event_handler(evh);
    }

    /// Handle an event.
    #[inline]
    pub fn handle_event(&self, event: Event) -> crate::Result<()> {
        match event.ty() {
            EventType::BoundsChanging => self.set_bounds_internal(
                *event.new_size().unwrap(),
                *Arc::downcast(event.arguments()[2].clone()).unwrap(),
            )?,
            EventType::TextChanging => {
                self.set_text_internal((*event.new_text().unwrap()).clone())?
            }
            EventType::AboutToPaint => self.repaint(None)?,
            _ => { /* do nothing */ }
        }

        let mut l = self.inner.lock();
        l.handle_event(&event)
    }

    /// Define which events should be handled by the window.
    #[inline]
    pub fn receive_events(&self, event_types: &[EventType]) -> crate::Result<()> {
        let mut he = self.handled_events.lock();
        // empty the hash set and fill it with event types
        he.clear();
        he.extend(event_types);

        let mut l = self.inner.lock();
        l.receive_events(event_types)
    }

    /// Does this window receive this event type?
    #[inline]
    pub fn receives_event(&self, event_type: &EventType) -> bool {
        let he = self.handled_events.lock();
        DEFAULT_EVENTS.contains(event_type) || he.contains(event_type)
    }

    /// Get the ID associated with this window.
    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Tell if this window is a top-level window.
    #[inline]
    pub fn is_top_level(&self) -> bool {
        self.inner.lock().is_top_level()
    }

    /// Get the bounds of this window.
    #[inline]
    pub fn bounds(&self) -> Rect<u32> {
        self.inner.lock().bounds()
    }

    fn set_bounds_internal(&self, bounds: Rect<u32>, backend: bool) -> crate::Result<()> {
        let mut l = self.inner.lock();

        self.instance.queue_event(Event::new(
            self,
            EventType::BoundsChanged,
            vec![Arc::new(l.set_bounds(bounds, backend)?), Arc::new(bounds)],
        ));
        Ok(())
    }

    /// Set the bounds of the window.
    #[inline]
    pub fn set_bounds(&self, bounds: Rect<u32>) -> crate::Result<()> {
        // this should just send a BoundsChaning event through, since that calls
        // set_bounds_internal when dispatched
        let l = self.inner.lock();
        self.instance.queue_event(Event::new(
            self,
            EventType::BoundsChanging,
            // Note: The Arc(true) at the end tells the evet
            vec![Arc::new(l.bounds()), Arc::new(bounds), Arc::new(true)],
        ));
        Ok(())
    }

    /// Display the window.
    #[inline]
    pub fn show(&self) -> crate::Result<()> {
        self.inner.lock().show()
    }

    /// Force a repaint operation on the window.
    #[inline]
    pub fn repaint(&self, bounds: Option<Rect<u32>>) -> crate::Result<()> {
        self.inner.lock().repaint(bounds)
    }
}

#[cfg(target_os = "linux")]
impl Window {
    /// The inner Flutterbug window.
    #[inline]
    pub(crate) fn inner_flutter_window(&self) -> ReadOnlyMappedMutexGuard<'_, flutterbug::Window> {
        ReadOnlyMappedMutexGuard::from_guard(self.inner.lock(), |i| i.inner_flutter_window())
    }

    #[inline]
    pub(crate) fn ic(&self) -> ReadOnlyMappedMutexGuard<'_, flutterbug::InputContext> {
        ReadOnlyMappedMutexGuard::from_guard(self.inner.lock(), |i| i.ic())
    }
}
