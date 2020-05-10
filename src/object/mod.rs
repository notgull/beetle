/* -----------------------------------------------------------------------------------
 * src/object/mod.rs - Declares the GuiObject trait, which is the root object for all
 *                     wrappers for native GUI objects.
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

use nalgebra::geometry::Point4;
use std::{
    fmt,
    ptr::{self, NonNull},
};

mod container;
pub use container::*;
mod factory;
pub use factory::*;
mod label;
pub use label::*;
mod textual;
pub use textual::*;
mod window;
pub use window::*;

#[cfg(target_os = "linux")]
mod linux;

pub trait GuiObject: fmt::Debug {
    /// The boundaries of the object
    fn bounds(&self) -> Point4<u32>;
    /// Set the boundaries of the object.
    fn set_bounds(&mut self, bounds: Point4<u32>) -> Result<(), crate::Error>;
    /// Set the object's parent
    fn set_parent(&mut self, parent: &dyn GuiObject) -> Result<(), crate::Error>;

    /// Gets the interior variable representing the window.
    #[cfg(target_os = "linux")]
    #[doc(hidden)]
    fn get_x11_window(&self) -> Option<x11::xlib::Window>;
    /// If this is a window, get the graphics context for this window.
    #[cfg(target_os = "linux")]
    #[doc(hidden)]
    fn get_x11_gc(&self) -> Option<NonNull<x11::xlib::_XGC>>;
    /// For X11 objects that require re-rendering.
    ///
    /// This should be a no-op for native widgets.
    #[cfg(target_os = "linux")]
    fn render(
        &self,
        display: &ptr::NonNull<x11::xlib::Display>,
        win: x11::xlib::Window,
        gc: NonNull<x11::xlib::_XGC>,
    ) -> Result<(), crate::Error>;
}

pub(crate) mod gui_object {
    pub use super::{
        ChildWindowBase, ContainerBase, GuiFactoryBase, GuiObject, GuiTextual, LabelBase,
        MainWindowBase, WindowBase,
    };
}

// object types
#[cfg(target_os = "linux")]
type _Label = linux::X11Label;
pub type Label = _Label;

#[cfg(target_os = "linux")]
type _MainWindow = linux::X11Window<linux::X11MainWindow>;
pub type MainWindow = _MainWindow;

#[cfg(target_os = "linux")]
type _ChildWindow = linux::X11Window<linux::X11ChildWindow>;
pub type ChildWindow = _ChildWindow;

#[cfg(target_os = "linux")]
type _GuiFactory = linux::X11Display;
pub type GuiFactory = _GuiFactory;
