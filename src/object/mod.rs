/* -----------------------------------------------------------------------------------
 * src/object/mod.rs - This file defines PeerObject traits and implementations. It
 *                     should import various items based upon the platform and loaded
 *                     libs.
 * beetle - Simple graphics framework for Rust
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

use crate::{GenericWidgetReference, Signal, Widget};
use euclid::default::Rect;
#[cfg(target_os = "linux")]
use flutterbug::{Event as X11Event, Window as X11Window};
use std::{
    boxed::Box,
    fmt,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

mod container;
pub use container::*;
mod textual;
pub use textual::*;
mod window;
pub use window::*;

#[cfg(target_os = "linux")]
#[path = "x11/mod.rs"]
mod x11_peer;

/// A peer object that acts as a rough wrapper around the internal API.
pub trait PeerObject: fmt::Debug {
    /// Set the bounds of this peer object.
    fn set_bounds(&mut self, bounds: Rect<u32>) -> Result<(), crate::Error>;
    /// Set the parent of this peer object.
    fn set_parent(&mut self, parent: &dyn PeerObject) -> Result<(), crate::Error>;

    /* platform-specific items */

    // x11

    /// Get a reference to the Flutterbug Window that this object wraps.
    #[cfg(target_os = "linux")]
    fn internal_x11_window(&self) -> &X11Window;
    /// Given an X11 event, respond to it an translate it into a Beetle event, if needed.
    #[cfg(target_os = "linux")]
    fn translate_x11_event(&mut self, xev: X11Event) -> Result<Vec<Arc<dyn Signal + 'static>>, crate::Error>;
}

/// A GUI Factory, or an object that is capable of producing peer objects and widgets.
pub trait GuiFactoryBase: fmt::Debug {
    /// The type of main window that this factory creates.
    type MainWindow: MainWindowBase;
    /// The type of child window that this factory creates.
    type ChildWindow: ChildWindowBase;
    /// The type of label that this factory creates.
    type Label: LabelBase;

    /// Create a new instance of this factory. Note that only one instance of this factory
    /// can exist per program, and an error will be thrown if one is created while another
    /// one still exists.
    fn new() -> Result<Self, crate::Error>
    where
        Self: Sized;
    /// Create a new instance of the main window peer. Only one of these can exist per
    /// application.
    fn main_window(&self, bounds: Rect<u32>) -> Result<Self::MainWindow, crate::Error>;
    /// Create a new instance of the child window peer.
    fn child_window(
        &self,
        parent: GenericWidgetReference,
        bounds: Rect<u32>,
    ) -> Result<Self::ChildWindow, crate::Error>;
    /// Create a new instance of a label.
    fn label(
        &self,
        parent: GenericWidgetReference,
        bounds: Rect<u32>,
        text: String,
    ) -> Result<Self::Label, crate::Error>;

    /// Register a widget into whatever context we need for events.
    fn post_creation<T: PeerObject>(&self, widget: Widget<T>) -> Result<(), crate::Error>;

    /// Run the main loop.
    fn main_loop(self) -> Result<(), crate::Error>;
}

#[cfg(target_os = "linux")]
type _GuiFactory = x11_peer::X11GuiFactory;
/// The default GUI factory object.
pub type GuiFactory = _GuiFactory;

#[cfg(target_os = "linux")]
type _MainWindow = x11_peer::X11Window<x11_peer::X11MainWindow>;
/// The default main window.
pub type MainWindow = _MainWindow;

#[cfg(target_os = "linux")]
type _ChildWindow = x11_peer::X11Window<x11_peer::X11ChildWindow>;
/// The default child window.
pub type ChildWindow = _ChildWindow;

#[cfg(target_os = "linux")]
type _Label = x11_peer::X11Label;
/// The default label.
pub type Label = _Label;
