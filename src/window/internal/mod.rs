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
use crate::{Event, EventType, Instance, Texture};
use alloc::string::String;
use euclid::default::Rect;

#[cfg(target_os = "linux")]
mod flutter;
#[cfg(target_os = "linux")]
use flutter::WindowInternal as _WindowInternal;

#[cfg(windows)]
mod porc;
#[cfg(windows)]
use porc::WindowInternal as _WindowInternal;

pub trait EventHandler = Fn(&Event) -> crate::Result<()> + Sync + Send + 'static;

pub(crate) fn default_event_handler(_ev: &Event) -> crate::Result<()> {
    log::debug!("Found event: {:?}", _ev);
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
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
        top_level: bool,
    ) -> crate::Result<Self>;

    /// Respond to an event.
    #[inline]
    fn handle_event(&self, event: &Event) -> crate::Result<()> {
        // run the default event handler after everything is done
        (self.event_handler())(event)
    }

    /// Receive certain types of events.
    fn receive_events(&self, events: &[EventType]) -> crate::Result<()>;

    /// Get the current event handler.
    fn event_handler(&self) -> &dyn EventHandler;

    /// Set the event handler.
    fn set_event_handler<F: EventHandler>(&mut self, evh: F);

    /// Get the text associated with this window. This can either be the title bar or
    /// the text contained within.
    fn text(&self) -> &str;

    /// Set the text associated with this window.
    fn set_text(&mut self, txt: String) -> crate::Result<String>;

    /// Get the texture used for the background of this window.
    fn background(&self) -> Option<&Texture>;

    /// Set the texture used for the background of this window.
    fn set_background(&mut self, texture: Option<Texture>);

    /// Take the background.
    fn take_background(&mut self) -> Option<Texture>;

    /// Get the bounds of this window.
    fn bounds(&self) -> Rect<u32>;

    /// Set the bounds of this window.
    fn set_bounds(&mut self, bounds: Rect<u32>, backend: bool) -> crate::Result<Rect<u32>>;

    /// Tell if the window is a top-level window (read: closing it means the whole application should be closed)
    fn is_top_level(&self) -> bool;

    /// Display the window.
    fn show(&self) -> crate::Result<()>;

    /// Repaint the window.
    fn repaint(&self, bounds: Option<Rect<u32>>) -> crate::Result<()>;
}

/// The internal window
pub type WindowInternal = _WindowInternal;
