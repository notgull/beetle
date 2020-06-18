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

use crate::{Event, EventType, GuiFactory, Instance, Texture};
use euclid::default::Rect;
use owning_ref::MutexGuardRef;
use std::{
    any::Any,
    boxed::Box,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex, RwLock},
};

mod id;
pub(crate) use id::*;

pub mod internal;
pub use internal::EventHandler;
pub use internal::*;

static INNER_WIDGET_MUTEX_FAIL: &'static str = "Unable to achieve lock on inner widget mutex";

/// A rectangle of pixels on the screen, in the most basic terms. This structure is actually
/// a cheaply copyable wrapper around the internal window object.
pub struct Window {
    inner: Arc<Mutex<WindowInternal>>,
    instance: Arc<Instance>,

    // make sure it owns any extra data
    _extra_data: Option<Arc<dyn Any>>,
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
            self.instance.clone(),
            self._extra_data.clone(),
        )
    }
}

impl Window {
    #[inline]
    pub(crate) fn from_raw(
        inner: Arc<Mutex<WindowInternal>>,
        instance: Arc<Instance>,
        extra_data: Option<Arc<dyn Any>>,
    ) -> Self {
        Self {
            inner,

            instance,
            _extra_data: extra_data,
        }
    }

    /// Create a new window.
    pub fn new(
        instance: &Arc<Instance>,
        parent: Option<&Window>,
        class_name: String,
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
    ) -> crate::Result<Self> {
        let instance = instance.clone();
        let internal =
            WindowInternal::new(&instance, parent, class_name, text, bounds, background)?;

        let this = Self::from_raw(Arc::new(Mutex::new(internal)), instance, None);
        // TODO: register
        Ok(this)
    }

    /// Get the text associated with this window.
    #[inline]
    pub fn text(&self) -> MutexGuardRef<'_, WindowInternal, str> {
        MutexGuardRef::new(self.inner.lock().expect(INNER_WIDGET_MUTEX_FAIL)).map(|i| i.text())
    }

    /// Set the text associated with this window. This will emit a TextChanged event.
    #[inline]
    pub fn set_text(&self, text: String) -> crate::Result<()> {
        let mut l = self.inner.lock().expect(INNER_WIDGET_MUTEX_FAIL);
        self.instance.queue_event(Event::new(
            self,
            EventType::TextChanged,
            vec![Arc::new(text.clone()), Arc::new(l.set_text(text))],
        ));
        Ok(())
    }

    /// Set the event handler.
    #[inline]
    pub fn set_event_handler<F: EventHandler>(&self, evh: F) {
        let mut l = self.inner.lock().expect(INNER_WIDGET_MUTEX_FAIL);
        l.set_event_handler(evh);
    }

    /// Handle an event.
    #[inline]
    pub fn handle_event(&self, event: Event) -> crate::Result<()> {
        let mut l = self.inner.lock().expect(INNER_WIDGET_MUTEX_FAIL);
        l.handle_event(event)
    }

    /// Get the ID associated with this window.
    #[inline]
    pub fn id(&self) -> usize {
        self.inner.lock().expect(INNER_WIDGET_MUTEX_FAIL).id()
    }
}

#[cfg(target_os = "linux")]
impl Window {
    pub(crate) fn inner_flutter_window(
        &self,
    ) -> MutexGuardRef<'_, WindowInternal, flutterbug::Window> {
        MutexGuardRef::new(self.inner.lock().expect(INNER_WIDGET_MUTEX_FAIL))
            .map(|i| i.inner_flutter_window())
    }

    pub(crate) fn ic(&self) -> MutexGuardRef<'_, WindowInternal, flutterbug::InputContext> {
        MutexGuardRef::new(self.inner.lock().expect(INNER_WIDGET_MUTEX_FAIL)).map(|i| i.ic())
    }
}
