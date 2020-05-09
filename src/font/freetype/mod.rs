/* -----------------------------------------------------------------------------------
 * src/font/freetype/mod.rs - A safe wrapper around the Freetype font type. This
 *                            should promote the safe use of Freetype resources.
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

mod error;
pub use error::*;

use crate::utils;
use freetype::freetype as ft;
use std::{path::Path, ptr};

/// The freetype core library object.
#[derive(Debug, Clone)]
pub struct FreetypeLibrary(ft::FT_Library);

impl FreetypeLibrary {
    #[inline]
    fn new() -> Self {
        let mut library: ft::FT_Library = ptr::null_mut();
        let error = unsafe { ft::FT_Init_FreeType(&mut library) };
        if error != 0 {
            panic!("Unable to load the FreeType library.")
        } else {
            Self(library)
        }
    }

    #[inline]
    fn library(&self) -> ft::FT_Library {
        self.0
    }
}

impl Drop for FreetypeLibrary {
    fn drop(&mut self) {
        unsafe { ft::FT_Done_FreeType(self.0) };
    }
}

unsafe impl Sync for FreetypeLibrary {}

lazy_static::lazy_static! {
    static ref FT_LIBRARY: FreetypeLibrary = FreetypeLibrary::new();
}

/// A wrapper around a FreeType font object.
#[derive(Debug, Clone)]
pub struct FreetypeFont(ft::FT_Face);

impl FreetypeFont {
    pub fn new<P: AsRef<Path>>(path: &P, em_size: u32) -> Result<Self, crate::Error> {
        let mut face: ft::FT_Face = ptr::null_mut();
        let mut error = unsafe {
            ft::FT_New_Face(
                FT_LIBRARY.library(),
                utils::to_cstring(path.as_ref().to_str().unwrap())?,
                0,
                &mut face,
            )
        };
        check_freetype_error(error)?;

        // also set the encoding
        error = unsafe { ft::FT_Select_Charmap(face, ft::FT_Encoding::FT_ENCODING_UNICODE) };
        check_freetype_error(error)?;

        Ok(Self(face))
    }

    #[inline]
    pub fn face(&self) -> ft::FT_Face {
        self.0
    }

    /// Loads a glpyh, as well as the glyph's (x, y) coordinates and advance vector.
    pub fn glyph<F>(&self, val: char, render_func: F) -> Result<(), crate::Error>
    where
        F: Fn(*const u8, u32, u32, u32, u32) -> Result<(), crate::Error>,
    {
        let val = val as ft::FT_ULong;
        let glyph_index = unsafe { ft::FT_Get_Char_Index(self.face(), val) };
        let mut error = unsafe { ft::FT_Load_Glyph(self.face(), glyph_index, 0) };
        check_freetype_error(error)?;
        error = unsafe {
            ft::FT_Render_Glyph(
                (*self.face()).glyph,
                ft::FT_Render_Mode_::FT_RENDER_MODE_LCD,
            )
        };
        check_freetype_error(error)?;

        // create a vector given our pixels
        let glyph = unsafe { *(*self.face()).glyph };
        let bitmap = glyph.bitmap;

        // run the render function with this
        render_func(
            bitmap.buffer,
            glyph.bitmap_left as u32,
            glyph.bitmap_top as u32,
            glyph.advance.x as u32,
            glyph.advance.y as u32,
        )
    }
}

impl Drop for FreetypeFont {
    fn drop(&mut self) {
        unsafe { ft::FT_Done_Face(self.face()) };
    }
}
