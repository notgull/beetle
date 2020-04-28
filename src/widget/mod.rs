/* -----------------------------------------------------------------------------------
 * src/widget/mod.rs - This file should defined the Widget struct. The Widget struct
 *                     is the item that represents all widgets on an internal level.
 *                     Widget should be defined as a wrapper around a shared reference
 *                     to the actual Widget object, in order to simplify some of the
 *                     semantics.
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

mod kind;
pub use kind::*;

use crate::Color;
use std::{ops::Deref, rc::Rc};

/// The internal representation of a Widget.
#[derive(Debug, Clone)]
pub struct WidgetInternal {
    kind: WidgetType,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    text: String,
    parent: Option<Widget>,
    children: Vec<Widget>,
    background_color: Option<Color>,
    foreground_color: Option<Color>,
}

/// A widget that represents a native object in the GUI.
#[derive(Debug, Clone)]
pub struct Widget {
    inner: Rc<WidgetInternal>,
}

impl Deref for Widget {
    type Target = WidgetInternal;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl WidgetInternal {
    // basic getters
    #[inline]
    pub fn kind(&self) -> WidgetType {
        self.kind
    }
    #[inline]
    pub fn x(&self) -> u32 {
        self.x
    }
    #[inline]
    pub fn y(&self) -> u32 {
        self.y
    }
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }
    #[inline]
    pub fn parent(&self) -> Option<&Widget> {
        (&self.parent).as_ref()
    }
    #[inline]
    pub fn children(&self) -> &[Widget] {
        &self.children
    }
    #[inline]
    pub fn background_color(&self) -> Option<Color> {
        self.background_color
    }
    #[inline]
    pub fn foreground_color(&self) -> Option<Color> {
        self.foreground_color
    }
}
