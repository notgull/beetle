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

use crate::{
    mutexes::{Mutex, RwLock, RwLockReadGuard},
    Event, EventData, EventType, Instance, Texture,
};
use alloc::{
    string::{String, ToString},
    sync::Arc,
};
use core::{
    any::Any,
    fmt,
    hash::{Hash, Hasher},
};
use euclid::default::Rect;
use hashbrown::HashSet;
#[cfg(feature = "std")]
use parking_lot::MappedRwLockReadGuard;
#[cfg(windows)]
use porcupine::{
    prelude::*,
    winapi::{
        shared::minwindef::{LPARAM, UINT, WPARAM},
        um::winuser,
    },
};
#[cfg(debug_assertions)]
use scopeguard::defer;

mod id;
pub(crate) use id::*;

mod internal;
pub use internal::EventHandler;
pub(crate) use internal::*;

// event types that are allowed no matter what
const DEFAULT_EVENTS: [EventType; 11] = [
    EventType::NoOp,
    EventType::AboutToPaint,
    EventType::Paint,
    EventType::TextChanging,
    EventType::TextChanged,
    EventType::Quit,
    EventType::Close,
    EventType::BoundsChanging,
    EventType::BoundsChanged,
    EventType::BackgroundChanging,
    EventType::BackgroundChanged,
];

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
///
/// # Example
///
/// ```no_run
/// use beetle::{Instance, Window};
/// use euclid::rect;
///
/// # fn main() -> beetle::Result<()> {
/// // a Window is created using Instance::create_window
/// let instance = Instance::new()?;
/// let my_window: Window = instance.create_window(
///                             None, // parent
///                             "Hello world!".to_string(), // associated text
///                             rect(0, 0, 100, 100), // bounds
///                             None, // background
///                             true, // is top level
///                         )?;
///
/// // if cloned, they will still refer to the same window
/// let cloned_window = my_window.clone();
/// // PartialEq and Eq are implemented for Window
/// assert_eq!(my_window, cloned_window);
///
/// # Ok(())
/// # }
/// ```
pub struct Window {
    inner: Arc<RwLock<WindowInternal>>,
    handled_events: Arc<Mutex<HashSet<EventType>>>,
    instance: Instance,
    id: usize,

    // make sure it owns any extra data
    _extra_data: Option<Arc<dyn Any>>,
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Window")
            .field("id", &self.id)
            .field("handled_events", &self.handled_events)
            .field("instance", &self.instance)
            .finish()
    }
}

impl Hash for Window {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.id().hash(h);
    }
}

impl PartialEq for Window {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
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
    /// The instance used to handle events and create this window.
    #[inline]
    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    /// Internal function to create a new Window.
    #[inline]
    pub(crate) fn from_raw(
        inner: Arc<RwLock<WindowInternal>>,
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
    ///
    /// This object will return a mutex guard containing the text associated with this window.
    /// Most often, the meaning of the text is dependent on the context of the window. For
    /// instance, a top level window will use its text as the title bar.
    ///
    /// This function returns a `ReadOnlyMappedMutexGuard`, which locks the mutex for the
    /// internal window. If this is used in threaded code, the mutex guard should be
    /// dropped ASAP after usage.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beetle::Instance;
    /// use euclid::rect;
    ///
    /// # fn main() -> beetle::Result<()> {
    /// const TEST_TEXT: &'static str = "Test!";
    /// let instance = Instance::new()?;
    /// let w = instance.create_window(None, TEST_TEXT.into_string(), rect(0, 0, 200, 100), None, true)?;
    ///
    /// let txt = w.text().into_string();
    /// assert_eq!(txt, TEST_TEXT.into_string());
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    pub fn text(&self) -> crate::Result<MappedRwLockReadGuard<'_, str>> {
        log::debug!("Providing read lock \"text\" from window id {}", self.id());
        self.inner
            .try_read()
            .map(|i| RwLockReadGuard::map(i, |i| i.text()))
            .ok_or_else(|| crate::Error::UnableToRead)
    }

    /// Set the text associated with this window. This will emit a TextChanged event.
    #[inline]
    pub fn set_text(&self, text: String) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        log::trace!("Locked read access for \"set_text\"");
        let l = self
            .inner
            .try_read()
            .ok_or_else(|| crate::Error::UnableToRead)?;
        #[cfg(debug_assertions)]
        defer!(log::trace!("Unlocked read access for \"set_text\""));

        self.instance.queue_event(Event::new(
            self,
            EventData::TextChanging {
                old: l.text().to_string(),
                new: text,
            },
        ));
        Ok(())
    }

    fn set_text_internal(&self, text: String) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        log::trace!("Locked write access for \"set_text_internal\"");
        let mut l = self
            .inner
            .try_write()
            .ok_or_else(|| crate::Error::UnableToWrite)?;
        #[cfg(debug_assertions)]
        defer!(log::trace!(
            "Unlocked write access for \"set_text_internal\""
        ));

        let cloned_text = text.clone();
        self.instance.queue_event(Event::new(
            self,
            EventData::TextChanged {
                old: l.set_text(text)?,
                new: cloned_text,
            },
        ));
        Ok(())
    }

    /// Set the event handler. This will not emit an event.
    ///
    /// The Event Handler is a function run after normal event processing is done.
    #[inline]
    pub fn set_event_handler<F: EventHandler>(&self, evh: F) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        log::trace!("Locked write access for \"set_event_handler\"");
        let mut l = self
            .inner
            .try_write()
            .ok_or_else(|| crate::Error::UnableToWrite)?;
        #[cfg(debug_assertions)]
        defer!(log::trace!(
            "Unlocked write access for \"set_event_handler\""
        ));

        l.set_event_handler(evh);
        Ok(())
    }

    /// Handle an event.
    #[inline]
    pub fn handle_event(&self, event: &Event) -> crate::Result<()> {
        match event.data() {
            EventData::BoundsChanging { old: _, ref new } => {
                let bools = *event.hidden_data::<(bool, bool)>().unwrap();
                self.set_bounds_internal(*new, bools.0, bools.1)?
            }
            EventData::TextChanging { old: _, ref new } => self.set_text_internal(new.clone())?,
            EventData::AboutToPaint => self.repaint(None)?,
            _ => { /* do nothing */ }
        }

        #[cfg(debug_assertions)]
        log::trace!("Locked read access for \"handle_event\"");
        let l = self
            .inner
            .try_read()
            .ok_or_else(|| crate::Error::UnableToRead)?;
        #[cfg(debug_assertions)]
        defer!(log::trace!("Unlocked read access for \"handle_event\""));

        l.handle_event(event)
    }

    /// Get the background for this window.
    #[cfg(feature = "std")]
    #[inline]
    pub fn background(&self) -> crate::Result<Option<MappedRwLockReadGuard<'_, Texture>>> {
        log::debug!(
            "Providing mutex lock \"background\" from window id {}",
            self.id()
        );
        let l = self
            .inner
            .try_read()
            .ok_or_else(|| crate::Error::UnableToRead)?;
        match l.background() {
            None => Ok(None),
            // Note: the unwrap() here will never panic. The read guard prevents write
            //       access to the background
            Some(_b) => Ok(Some(RwLockReadGuard::map(l, |l| l.background().unwrap()))),
        }
    }

    /// Define which events should be handled by the window.
    #[inline]
    pub fn receive_events(&self, event_types: &[EventType]) -> crate::Result<()> {
        let mut he = self.handled_events.lock();
        // empty the hash set and fill it with event types
        he.clear();
        he.extend(event_types);

        #[cfg(debug_assertions)]
        log::trace!("Locked read access for \"receive_events\"");
        let l = self
            .inner
            .try_read()
            .ok_or_else(|| crate::Error::UnableToRead)?;
        #[cfg(debug_assertions)]
        defer!(log::trace!("Unlocked read access for \"receive_events\""));

        l.receive_events(event_types)
    }

    /// Does this window receive this event type?
    #[inline]
    pub fn receives_event(&self, event_type: &EventType) -> bool {
        // don't bother locking the mutex if the default events already contains the type
        if DEFAULT_EVENTS.contains(event_type) {
            return true;
        }

        let he = self.handled_events.lock();
        he.contains(event_type)
    }

    /// Get the ID associated with this window.
    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Tell if this window is a top-level window.
    #[inline]
    pub fn is_top_level(&self) -> crate::Result<bool> {
        #[cfg(debug_assertions)]
        log::trace!("Locked read access for \"is_top_level\"");
        #[cfg(debug_assertions)]
        defer!(log::trace!("Unlocked read access for \"is_top_level\""));

        Ok(self
            .inner
            .try_read()
            .ok_or_else(|| crate::Error::UnableToRead)?
            .is_top_level())
    }

    /// Get the bounds of this window.
    #[inline]
    pub fn bounds(&self) -> crate::Result<Rect<u32>> {
        #[cfg(debug_assertions)]
        log::trace!("Locked read access for \"bounds\"");
        #[cfg(debug_assertions)]
        defer!(log::trace!("Unlocked read access for \"bounds\""));

        self.inner
            .try_read()
            .ok_or_else(|| crate::Error::UnableToRead)
            .map(|i| i.bounds())
    }

    fn set_bounds_internal(
        &self,
        bounds: Rect<u32>,
        backend: bool,
        enqueue: bool,
    ) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        log::trace!("Locked write access for \"set_bounds_internal\"");
        let mut l = self
            .inner
            .try_write()
            .ok_or_else(|| crate::Error::UnableToWrite)?;
        #[cfg(debug_assertions)]
        defer!(log::trace!(
            "Unlocked write access for \"set_bounds_internal\""
        ));

        let old_bounds = l.set_bounds(bounds, backend)?;

        if enqueue {
            log::trace!("Queueing new BoundsChanged event");
            self.instance.queue_event(Event::new(
                self,
                EventData::BoundsChanged {
                    old: old_bounds,
                    new: bounds,
                },
            ));
        }
        Ok(())
    }

    /// Set the bounds of the window.
    #[inline]
    pub fn set_bounds(&self, bounds: Rect<u32>) -> crate::Result<()> {
        // this should just send a BoundsChanging event through, since that calls
        // set_bounds_internal when dispatched
        #[cfg(debug_assertions)]
        log::trace!("Locked read access for \"set_bounds\"");
        let l = self
            .inner
            .try_read()
            .ok_or_else(|| crate::Error::UnableToRead)?;
        #[cfg(debug_assertions)]
        defer!(log::trace!("Unlocked read access for \"set_bounds\""));

        let mut ev = Event::new(
            self,
            EventData::BoundsChanging {
                old: l.bounds(),
                new: bounds,
            },
        );

        // Note: The (true, true) tells the event handler to both
        // set this on the X11 backend and release a BoundsChanged event
        ev.set_hidden_data((true, true));
        self.instance.queue_event(ev);

        Ok(())
    }

    /// Display the window.
    #[inline]
    pub fn show(&self) -> crate::Result<()> {
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                use core::mem;

                // clone the window out of the lock
                let l = self.inner_window()?;
                let w = l.inner_porc_window().weak_reference();
                mem::drop(l);

                w.show(porcupine::CmdShow::Show);
                w.update()?;
                Ok(())
            } else {
                self.inner
                    .try_read()
                    .ok_or_else(|| crate::Error::UnableToRead)?
                    .show()
            }
        }
    }

    /// Force a repaint operation on the window.
    #[inline]
    pub fn repaint(&self, bounds: Option<Rect<u32>>) -> crate::Result<()> {
        self.inner
            .try_read()
            .ok_or_else(|| crate::Error::UnableToRead)?
            .repaint(bounds)
    }
}

impl Window {
    #[inline]
    pub(crate) fn inner_window(
        &self,
    ) -> crate::Result<RwLockReadGuard<'_, internal::WindowInternal>> {
        self.inner
            .try_read()
            .ok_or_else(|| crate::Error::UnableToRead)
    }
}

#[cfg(windows)]
impl Window {
    #[inline]
    pub(crate) fn store_old_bounds(&self) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        log::trace!("Locked write access for \"store_old_bounds\"");
        #[cfg(debug_assertions)]
        defer!(log::trace!(
            "Unlocked write access for \"store_old_bounds\""
        ));

        self.inner
            .try_write()
            .ok_or_else(|| crate::Error::UnableToWrite)?
            .store_old_bounds();
        Ok(())
    }

    #[inline]
    pub(crate) fn take_old_bounds(&self) -> crate::Result<Option<Rect<u32>>> {
        #[cfg(debug_assertions)]
        log::trace!("Locked write access for \"take_old_bounds\"");
        #[cfg(debug_assertions)]
        defer!(log::trace!("Unlocked write access for \"take_old_bounds\""));

        Ok(self
            .inner
            .try_write()
            .ok_or_else(|| crate::Error::UnableToRead)?
            .take_old_bounds())
    }
}
