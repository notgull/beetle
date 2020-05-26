/* -----------------------------------------------------------------------------------
 * src/font.rs - Describes a simple text style. Things like font family, size, and
 *               other items. Note that some of these are contained within the FontKit
 *               "Font" object. This should use an Arc to hold some items, in order to
 *               be cheaply copyable.
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

use font_kit::{
    canvas::{Canvas, Format, RasterizationOptions},
    family_name::FamilyName,
    font::Font as FKFont,
    hinting::HintingOptions,
    properties::{Properties, Style},
    source::SystemSource,
};
use pathfinder_geometry::{
    transform2d::Transform2F,
    vector::{Vector2F, Vector2I},
};
use std::sync::Arc;

/// A cheaply clonable wrapper around a font, including size.
#[derive(Debug)]
pub struct Font {
    inner: Arc<FKFont>,
    size: u32,
}

impl Clone for Font {
    fn clone(&self) -> Self {
        Self::from_raw(self.inner.clone(), self.size)
    }
}

impl Font {
    /// Create a new font from its raw components.
    #[inline]
    pub(crate) fn from_raw(inner: Arc<FKFont>, size: u32) -> Self {
        Self { inner, size }
    }

    /// Create a new font based on an existing font-kit font and a size.
    #[inline]
    pub fn new(font: FKFont, size: u32) -> Self {
        Self::from_raw(Arc::new(font), size)
    }

    /// Get a new font based on the font name and the font properties.
    pub fn by_name_and_properties(
        name: String,
        italic: bool,
        weight: u32,
        size: u32,
    ) -> Result<Self, crate::Error> {
        Ok(Self::new(
            SystemSource::new()
                .select_best_match(
                    &[FamilyName::Title(name)],
                    &Properties::new().style(if italic { Style::Italic } else { Style::Normal }),
                )?
                .load()?,
            size,
        ))
    }
}
