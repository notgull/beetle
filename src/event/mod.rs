/* -----------------------------------------------------------------------------------
 * src/event/mod.rs - An event from the event loop.
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

use crate::{Graphics, KeyInfo, MouseButton, Texture, Window};
use alloc::{string::String, sync::Arc, vec, vec::Vec};
use core::{any::Any, fmt, option::Option};
use euclid::default::{Point2D, Rect};

#[cfg(target_os = "linux")]
mod flutter;
#[cfg(windows)]
mod porc;

/// Types of events deployed from Beetle.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    /// Nothing is happening. Often used as a transport for event data.
    NoOp,
    /// A key has been pressed.
    KeyDown,
    /// A key has been released.
    KeyUp,
    /// The window is about to be repainted.
    AboutToPaint,
    /// The window is being repainted.
    Paint,
    /// The text of a window is currently changing.
    TextChanging,
    /// The text of a window has been changed.
    TextChanged,
    /// The application is closing.
    Quit,
    /// A single window is closing.
    Close,
    /// The window's bounds are being changed.
    BoundsChanging,
    /// The window's bounds have changed.
    BoundsChanged,
    /// The window's background is being changed.
    BackgroundChanging,
    /// The window's background has been changed.
    BackgroundChanged,
    /// The window has had a mouse button depressed on it.
    MouseButtonDown,
    /// The window has had a mouse button released on it.
    MouseButtonUp,
    /// A manual, integer event.
    Integer(usize),
    /// A manual, string event.
    Str(&'static str),
}

/// Types of data deployed from Beetle.
#[derive(Debug)]
pub enum EventData {
    /// Nothing is happening. Often used as a transport for event data.
    NoOp,
    /// A key has been pressed.
    KeyDown(KeyInfo, Option<Point2D<u32>>),
    /// A key has been released.
    KeyUp(KeyInfo, Option<Point2D<u32>>),
    /// The window is about to be repainted.
    AboutToPaint,
    /// The window is being repainted.
    Paint(Graphics),
    /// The text of a window is currently changing.
    TextChanging { old: String, new: String },
    /// The text of a window has been changed.
    TextChanged { old: String, new: String },
    /// The application is closing.
    Quit,
    /// A single window is closing.
    Close,
    /// The window's bounds are being changed.
    BoundsChanging { old: Rect<u32>, new: Rect<u32> },
    /// The window's bounds have changed.
    BoundsChanged { old: Rect<u32>, new: Rect<u32> },
    /// The window's background is being changed.
    BackgroundChanging {
        old: Option<Texture>,
        new: Option<Texture>,
    },
    /// The window's background has been changed.
    BackgroundChanged,
    /// The window has had a mouse button depressed on it.
    MouseButtonDown(Point2D<u32>, MouseButton),
    /// The window has had a mouse button released on it.
    MouseButtonUp(Point2D<u32>, MouseButton),
    /// A manual, integer event.
    Integer(usize),
    /// A manual, string event.
    Str(&'static str),
}

impl EventData {
    /// Get the type of the event from the data.
    #[inline]
    pub fn ty(&self) -> EventType {
        match self {
            EventData::NoOp => EventType::NoOp,
            EventData::KeyDown(ref _k, ref _o) => EventType::KeyDown,
            EventData::KeyUp(ref _k, ref _o) => EventType::KeyUp,
            EventData::AboutToPaint => EventType::AboutToPaint,
            EventData::Paint(ref _g) => EventType::Paint,
            EventData::TextChanging { ref old, ref new } => EventType::TextChanging,
            EventData::TextChanged { ref old, ref new } => EventType::TextChanged,
            EventData::Quit => EventType::Quit,
            EventData::Close => EventType::Close,
            EventData::BoundsChanging { ref old, ref new } => EventType::BoundsChanging,
            EventData::BoundsChanged { ref old, ref new } => EventType::BoundsChanged,
            EventData::BackgroundChanging { ref old, ref new } => EventType::BackgroundChanging,
            EventData::BackgroundChanged => EventType::BackgroundChanged,
            EventData::MouseButtonDown(ref _p, ref _b) => EventType::MouseButtonDown,
            EventData::MouseButtonUp(ref _p, ref _b) => EventType::MouseButtonUp,
            EventData::Integer(id) => EventType::Integer(*id),
            EventData::Str(id) => EventType::Str(id),
        }
    }
}

/// An event receieved from the event loop.
pub struct Event {
    target_window: Window, // cloned reference
    data: EventData,
    arguments: Vec<Arc<dyn Any + Send + Sync + 'static>>,
    hidden_data: Option<Arc<dyn Any + Send + Sync + 'static>>,
    needs_quit: bool,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Event")
            .field("target_window", &self.target_window)
            .field("data", &self.data)
            .field("needs_quit", &self.needs_quit)
            .finish()
    }
}

impl Event {
    /// Create a new event from its raw parts.
    #[inline]
    pub fn new(target_window: &Window, data: EventData) -> Self {
        Self {
            target_window: target_window.clone(),
            data,
            arguments: vec![],
            hidden_data: None,
            needs_quit: false,
        }
    }

    #[inline]
    pub(crate) fn set_hidden_data<T: Any + Send + Sync + 'static>(&mut self, data: T) {
        self.hidden_data = Some(Arc::new(data));
    }

    #[inline]
    pub(crate) fn hidden_data<T: Any + Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        match self.hidden_data {
            None => None,
            Some(ref hd) => Arc::downcast(hd.clone()).ok(),
        }
    }

    /// Get the type of the event.
    #[inline]
    pub fn ty(&self) -> EventType {
        self.data.ty()
    }

    /// Get the data of the event.
    #[inline]
    pub fn data(&self) -> &EventData {
        &self.data
    }

    /// Get the window that this event targets.
    #[inline]
    pub fn window(&self) -> &Window {
        &self.target_window
    }

    /// Dispatch its event to the system handling source.
    #[inline]
    pub fn dispatch(&self) -> crate::Result<()> {
        self.window().handle_event(self)
    }

    /// Tell if the event requires the application to exit.
    #[inline]
    pub fn is_exit_event(&self) -> bool {
        self.needs_quit
    }

    /// Set whether or not this is a quit event.
    #[inline]
    pub fn set_is_exit_event(&mut self, is_quit: bool) {
        self.needs_quit = is_quit;
    }
}
