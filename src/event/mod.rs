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

use crate::{KeyInfo, MouseButton, Window};
use alloc::{string::String, sync::Arc, vec::Vec};
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
    /// A carrier for a Win32 message.
    MessageCarrier,
    /// A manual, integer event.
    Integer(usize),
    /// A manual, string event.
    Str(&'static str),
}

/// An event receieved from the event loop.
pub struct Event {
    target_window: Window, // cloned reference
    ty: EventType,
    arguments: Vec<Arc<dyn Any + Send + Sync + 'static>>,
    needs_quit: bool,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Event")
            .field("target_window", &self.target_window)
            .field("ty", &self.ty)
            .field("needs_quit", &self.needs_quit)
            .finish()
    }
}

impl Event {
    /// Create a new event from its raw parts.
    #[inline]
    pub fn new(
        target_window: &Window,
        ty: EventType,
        arguments: Vec<Arc<dyn Any + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            target_window: target_window.clone(),
            ty,
            arguments,
            needs_quit: false,
        }
    }

    /// Get the type of the event.
    #[inline]
    pub fn ty(&self) -> EventType {
        self.ty
    }

    /// Get the window that this event targets.
    #[inline]
    pub fn window(&self) -> &Window {
        &self.target_window
    }

    /// Get the arguments used in this event.
    #[inline]
    pub fn arguments(&self) -> &[Arc<dyn Any + Send + Sync + 'static>] {
        &self.arguments
    }

    /// Helper Function to downcast an Arc to get information regarding an event.
    #[inline]
    fn event_info<T: Any + Send + Sync + 'static>(
        &self,
        event_types: &[EventType],
        index: usize,
    ) -> Option<Arc<T>> {
        if event_types.contains(&self.ty) {
            Arc::downcast(self.arguments.get(index)?.clone()).ok()
        } else {
            None
        }
    }

    /// If this is a KeyEvent, get the associated KeyInfo.
    #[inline]
    pub fn key(&self) -> Option<Arc<KeyInfo>> {
        self.event_info(&[EventType::KeyDown, EventType::KeyUp], 0)
    }

    /// If this is a key event, get where the mouse was when the key was pressed, if applicable.
    #[inline]
    pub fn press_location(&self) -> Option<Arc<Option<Point2D<u32>>>> {
        self.event_info(&[EventType::KeyDown, EventType::KeyUp], 1)
    }

    /// If this is a resize event, get the old size.
    #[inline]
    pub fn old_bounds(&self) -> Option<Arc<Rect<u32>>> {
        self.event_info(&[EventType::BoundsChanging, EventType::BoundsChanged], 0)
    }

    /// If this is a resize event, get the new size.
    #[inline]
    pub fn new_bounds(&self) -> Option<Arc<Rect<u32>>> {
        self.event_info(&[EventType::BoundsChanging, EventType::BoundsChanged], 1)
    }

    /// If this is a text change event, get the old text.
    #[inline]
    pub fn old_text(&self) -> Option<Arc<String>> {
        self.event_info(&[EventType::TextChanging, EventType::TextChanged], 0)
    }

    /// If this is a text change event, get the new text.
    #[inline]
    pub fn new_text(&self) -> Option<Arc<String>> {
        self.event_info(&[EventType::TextChanging, EventType::TextChanged], 1)
    }

    /// If this is a mouse button event, get the point where the mouse clicked.
    #[inline]
    pub fn click_location(&self) -> Option<Arc<Point2D<u32>>> {
        self.event_info(&[EventType::MouseButtonUp, EventType::MouseButtonDown], 0)
    }

    /// If this is a mouse button event, get the button that has been clicked.
    #[inline]
    pub fn click_button(&self) -> Option<Arc<MouseButton>> {
        self.event_info(&[EventType::MouseButtonUp, EventType::MouseButtonDown], 1)
    }

    /// Dispatch its event to the system handling source.
    #[inline]
    pub fn dispatch(self) -> crate::Result<()> {
        let win = self.target_window.clone();
        win.handle_event(self)
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
