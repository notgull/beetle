/* -----------------------------------------------------------------------------------
 * src/font/freetype/error.rs - Allows for the enumification of Freetype errors.
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

use freetype::freetype as ft;
use thiserror::Error;

/// An error that occurred during FreeType code execution.
///
/// See also: https://www.freetype.org/freetype2/docs/reference/ft2-error_code_values.html
#[derive(Debug, Error)]
pub enum FreetypeError {
    #[error("Unable to open resource")]
    CannotOpenResource,
    #[error("Invalid file format")]
    InvalidFileFormat,
    #[error("Invalid FreeType version")]
    InvalidVersion,
    #[error("Module version is too low")]
    LowerModuleVersion,
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Unimplemented feature")]
    UnimplementedFeature,
    #[error("Invalid table")]
    InvalidTable,
    #[error("Invalid offset")]
    InvalidOffset,
    #[error("Array allocation size too large")]
    ArrayTooLarge,
    #[error("Missing module")]
    MissingModule,
    #[error("Missing property")]
    MissingProperty,

    #[error("Invalid glyph index")]
    InvalidGlyphIndex,
    #[error("Invalid character code")]
    InvalidCharacterCode,
    #[error("Unsupported glyph image format")]
    InvalidGlyphFormat,
    #[error("Cannot render glyph format")]
    CannotRenderGlyph,
    #[error("Invalid outline")]
    InvalidOutline,
    #[error("Invalid composite glyph")]
    InvalidComposite,
    #[error("Too many hints")]
    TooManyHints,
    #[error("Invalid pixel size")]
    InvalidPixelSize,

    #[error("Invalid object handle")]
    InvalidHandle,
    #[error("Invalid library handle")]
    InvalidLibraryHandle,
    #[error("Invalid driver handle")]
    InvalidDriverHandle,
    #[error("Invalid face handle")]
    InvalidFaceHandle,
    #[error("Invalid size handle")]
    InvalidSizeHandle,
    #[error("Invalid glyph slot handle")]
    InvalidSlotHandle,
    #[error("Invalid charmap handle")]
    InvalidCharMapHandle,
    #[error("Invalid cache manager handle")]
    InvalidCacheHandle,
    #[error("Invalid stream handle")]
    InvalidStreamHandle,

    #[error("Too many modules")]
    TooManyDrivers,
    #[error("Too many extensions")]
    TooManyExtensions,

    #[error("Out of memory")]
    OutOfMemory,
    #[error("Unlisted object")]
    UnlistedObject,

    #[error("Cannot open stream")]
    CannotOpenStream,
    #[error("Invalid stream seek")]
    InvalidStreamSeek,
    #[error("Invalid stream skip")]
    InvalidStreamSkip,
    #[error("Invalid stream read")]
    InvalidStreamRead,
    #[error("Invalid stream operation")]
    InvalidStreamOperation,
    #[error("Invalid frame operation")]
    InvalidFrameOperation,
    #[error("Nested frame access")]
    NestedFrameAccess,
    #[error("Invalid frame read")]
    InvalidFrameRead,

    #[error("Raster uninitialized")]
    RasterUninitialized,
    #[error("Raster corrupted")]
    RasterCorrupted,

    #[error("Unknown error")]
    Unknown(ft::FT_Error),
}

#[inline]
fn convert_to_rerror(error: freetype::FT_Error) -> FreetypeError {
    type fte = FreetypeError;

    // TODO: a good match statement
    fte::Unknown(error)
}

/// If the number represents a Freetype error, return a Rust error.
#[inline]
pub fn check_freetype_error(error: freetype::FT_Error) -> Result<(), FreetypeError> {
    const FT_ERR_OK: i32 = ft::FT_Err_Ok as i32;

    match error {
        FT_ERR_OK => Ok(()),
        _ => Err(convert_to_rerror(error)),
    }
}
