/* -----------------------------------------------------------------------------------
 * src/object/factory.rs - The factory object that contains functions for producing
 *                         various types of GUI objects.
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

use super::{ChildWindowBase, GuiObject, LabelBase, MainWindow, MainWindowBase, WindowBase};
use crate::{Font, Widget};
use nalgebra::Point4;

pub trait GuiFactoryBase: Sized {
    type Label: LabelBase;
    type MainWindow: MainWindowBase;
    type ChildWindow: ChildWindowBase;

    /// Create a new GuiFactory.
    fn new() -> Result<Self, crate::Error>;
    /// Create a new main window.
    fn main_window(
        &self,
        bounds: Point4<u32>,
        title: &str,
    ) -> Result<Self::MainWindow, crate::Error>;
    /// Create a new label.
    fn label<T: GuiObject>(
        &self,
        parent: &T,
        bounds: Point4<u32>,
        text: &str,
        font: &Font,
    ) -> Result<Self::Label, crate::Error>;
    /// Create a new child window.
    fn child_window<T: WindowBase>(
        &self,
        parent: &T,
        bounds: Point4<u32>,
        title: &str,
    ) -> Result<Self::ChildWindow, crate::Error>;
    /// Begin the main loop.
    fn main_loop(self, window: Widget<MainWindow>) -> Result<(), crate::Error>;
}
