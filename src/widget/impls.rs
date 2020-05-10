/* -------------------------------------------------------------------------------
 * src/widget/impls.rs - Object specific methods.
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

use super::{GenericWidget, Widget, WidgetInternal};
use crate::{
    object::{
        ChildWindow, ContainerBase, GuiFactory, GuiFactoryBase, GuiObject, GuiTextual, Label,
        MainWindow, WindowBase,
    },
    Font,
};
use nalgebra::geometry::Point4;
use owning_ref::RefRef;

impl<Inner: GuiTextual + 'static> Widget<Inner> {
    /// Get the text that this widget is displaying.
    #[inline]
    pub fn text(&self) -> Result<RefRef<'_, WidgetInternal<Inner>, str>, crate::Error> {
        Ok(RefRef::new(self.internal().try_borrow()?).map(|i| i.text()))
    }
    /// Set the text that this widget is displaying.
    #[inline]
    pub fn set_text(&self, val: String) -> Result<(), crate::Error> {
        self.internal().try_borrow_mut()?.set_text(val)
    }
}

impl<Inner: WindowBase + 'static> Widget<Inner> {
    /// Get the title used for this window.
    #[inline]
    pub fn title(&self) -> Result<RefRef<'_, WidgetInternal<Inner>, str>, crate::Error> {
        Ok(RefRef::new(self.internal().try_borrow()?).map(|i| i.text()))
    }

    /// Set the title used for this window.
    #[inline]
    pub fn set_title(&self, val: String) -> Result<(), crate::Error> {
        self.internal().try_borrow_mut()?.set_title(val)
    }

    /// Display this window.
    #[inline]
    pub fn display(&self) -> Result<(), crate::Error> {
        self.inner()?.display()
    }
}

// instantiation only
impl Widget<Label> {
    /// Create a new label.
    #[inline]
    pub fn new_label<T: ContainerBase>(
        factory: &GuiFactory,
        parent: &Widget<T>,
        bounds: Point4<u32>,
        text: String,
        font: Option<&Font>,
    ) -> Result<Self, crate::Error> {
        let inner = factory.label(&*parent.inner()?, bounds, &text, font)?;
        Ok(Self::from_inner(inner, bounds, text))
    }
}

impl Widget<MainWindow> {
    /// Create a new Main Window.
    #[inline]
    pub fn new_main_window(
        factory: &GuiFactory,
        bounds: Point4<u32>,
        title: String,
    ) -> Result<Self, crate::Error> {
        let inner = factory.main_window(bounds, &title)?;
        Ok(Self::from_inner(inner, bounds, title))
    }
}

impl Widget<ChildWindow> {
    /// Create a new Child Window.
    #[inline]
    pub fn new_child_window<T: WindowBase>(
        factory: &GuiFactory,
        window: Widget<T>,
        bounds: Point4<u32>,
        title: String,
    ) -> Result<Self, crate::Error> {
        let inner = factory.child_window(&*window.inner()?, bounds, &title)?;
        Ok(Self::from_inner(inner, bounds, title))
    }
}
