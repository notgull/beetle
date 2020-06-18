/* -----------------------------------------------------------------------------------
 * src/window/internal/mod.rs - Traits and definitions for the internal window.
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

pub(crate) use super::{unique_id, Window};
use crate::{Event, Instance, Texture};
use euclid::default::Rect;
use std::hash::{Hash, Hasher};

#[cfg(target_os = "linux")]
mod flutter;
#[cfg(target_os = "linux")]
use flutter::WindowInternal as _WindowInternal;

#[cfg(windows)]
mod porc;
#[cfg(windows)]
use porc::WindowInternal as _WindowInternal;

pub trait EventHandler = Fn(Event) -> crate::Result<()> + Sync + Send + 'static;

pub(crate) fn default_event_handler(_ev: Event) -> crate::Result<()> {
    Ok(())
}

/// Public functions of an internal window.
pub trait GenericWindowInternal: Sized {
    /// Get a unique ID identifying this window.
    fn id(&self) -> usize;

    /// Create a new version of this window.
    fn new(
        instance: &Instance,
        parent: Option<&Window>,
        class_name: String,
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
    ) -> crate::Result<Self>;

    /// Respond to an event.
    fn handle_event(&mut self, event: Event) -> crate::Result<()> {
        // TODO: match evvent to determine handling

        // run the default event handler after everything is done
        (self.event_handler())(event)
    }

    /// Get the current event handler.
    fn event_handler(&self) -> &dyn EventHandler;

    /// Set the event handler.
    fn set_event_handler<F: EventHandler>(&mut self, evh: F);

    /// Get the text associated with this window. This can either be the title bar or
    /// the text contained within.
    fn text(&self) -> &str;

    /// Set the text associated with this window.
    fn set_text(&mut self, txt: String) -> String;
}

impl Hash for WindowInternal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for WindowInternal {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }

    fn ne(&self, other: &Self) -> bool {
        self.id() != other.id()
    }
}

impl Eq for WindowInternal {}

/// The internal window
pub type WindowInternal = _WindowInternal;
