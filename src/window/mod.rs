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
mod internal;
#[cfg(feature = "expose_internal")]
pub use internal::*;
#[cfg(not(feature = "expose_internal"))]
pub(crate) use internal::*;

use crate::{
    mutexes::{Mutex, RwLock},
    EventType, Pixel, Texture,
};
use alloc::{string::String, sync::Arc};
use core::{
    fmt,
    hash::{Hash, Hasher},
};
use euclid::Rect;
use hashbrown::HashSet;

/// Miscellaneous properties a window can hold.
pub(crate) struct WindowProperties {
    text: String,
    bounds: Rect<u32, Pixel>,
    background: Option<Texture>,
}

impl WindowProperties {
    pub fn new(text: String, bounds: Rect<u32, Pixel>, background: Option<Texture>) -> Self {
        Self {
            text,
            bounds,
            background,
        }
    }
}

struct WindowInner {
    // the backend window
    // write access isn't needed that often, but read access isso we use an RwLock
    backend: RwLock<internal::InternalWindow>,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Window")
            .field("id", self.0.id)
            .field("instance", self.1)
            .finish()
    }
}

impl Hash for Window {
    #[inline]
    fn hash<H: Hasher>(&self, h: &mut H) {
        Hash::hash(self.id, h)
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
        backend: RwLock<internal::InternalWindow>,
        properties: Mutex<WindowProperties>,
        instance: Instance,
    ) -> Self {
        Self(
            WindowInner {
                backend,
                properties,
                events_received: RwLock::new(HashSet::new()),
                id: id::unique_id(),
            },
            instance,
        )
    }

    /// Get the ID for this widget.
    #[inline]
    pub fn id(&self) -> usize {
        self.0.id
    }
}
