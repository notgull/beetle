/* -----------------------------------------------------------------------------------
 * src/pixel_buffer.rs - A buffer to hold pixels, mostly for images.
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

use crate::Color;
use alloc::{boxed::Box, vec::Vec, sync::Arc};
use core::fmt;
use euclid::default::{Point2D, Size2D};

/// Possible formats for the pixel buffer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Format {
    Grayscale,
    Rgb,
    Rgba,
}

// gets colors from the pixel buffer
trait Formatter {
    fn bytes_per_color(&self) -> usize;
    fn get_color(&self, bytes: &[u8]) -> crate::Result<Color>;
}

// grayscale formatter
#[derive(Debug, Copy, Clone)]
struct GrayscaleFormatter;

impl Formatter for GrayscaleFormatter {
    #[inline]
    fn bytes_per_color(&self) -> usize {
        1
    }

    #[inline]
    fn get_color(&self, bytes: &[u8]) -> crate::Result<Color> {
        Ok(Color::from_rgba(
            bytes[0],
            bytes[0],
            bytes[0],
            core::u8::MAX,
        ))
    }
}

// rgb formatter
#[derive(Debug, Copy, Clone)]
struct RgbFormatter;

impl Formatter for RgbFormatter {
    #[inline]
    fn bytes_per_color(&self) -> usize {
        3
    }
    #[inline]
    fn get_color(&self, bytes: &[u8]) -> crate::Result<Color> {
        Ok(Color::from_rgba(
            bytes[0],
            bytes[1],
            bytes[2],
            core::u8::MAX,
        ))
    }
}

// rgba formatter
#[derive(Debug, Copy, Clone)]
struct RgbaFormatter;

impl Formatter for RgbaFormatter {
    #[inline]
    fn bytes_per_color(&self) -> usize {
        4
    }
    #[inline]
    fn get_color(&self, bytes: &[u8]) -> crate::Result<Color> {
        Ok(Color::from_rgba(bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}

// the various types of formatters
#[derive(Debug, Copy, Clone)]
enum ColorFetcher {
    Grayscale(GrayscaleFormatter),
    Rgb(RgbFormatter),
    Rgba(RgbaFormatter),
}

impl From<Format> for ColorFetcher {
    #[inline]
    fn from(f: Format) -> Self {
        match f {
            Format::Grayscale => Self::Grayscale(GrayscaleFormatter),
            Format::Rgb => Self::Rgb(RgbFormatter),
            Format::Rgba => Self::Rgba(RgbaFormatter),
        }
    }
}

impl ColorFetcher {
    // get the formatter used by this fetcher
    fn formatter(&self) -> &dyn Formatter {
        match self {
            Self::Grayscale(ref g) => g,
            Self::Rgb(ref r) => r,
            Self::Rgba(ref a) => a,
        }
    }

    // get the format involved with this color fetcher
    fn format(&self) -> Format {
        match self {
            Self::Grayscale(_g) => Format::Grayscale,
            Self::Rgb(_r) => Format::Rgb,
            Self::Rgba(_a) => Format::Rgba,
        }
    }
}

/// A read-only buffer for pixels. This is cheaply clonable.
pub struct PixelBuffer {
    data: Arc<[u8]>,
    size: Size2D<usize>,
    fetcher: ColorFetcher,
}

impl Clone for PixelBuffer {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            size: self.size.clone(),
            fetcher: self.fetcher.clone(),
        }
    }
}

impl fmt::Debug for PixelBuffer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PixelBuffer")
            .field("size", &self.size)
            .field("fetcher", &self.fetcher)
            .finish()
    }
}

/// An iterator over the pixels in the pixel buffer.
pub struct PixelBufferIter<'a> {
    data: &'a [u8],
    current_pix: usize,
    fetcher: ColorFetcher,
}

impl<'a> Iterator for PixelBufferIter<'a> {
    type Item = Color;

    fn next(&mut self) -> Option<Color> {
        // how many bytes to get?
        let num_next_bytes = self.fetcher.formatter().bytes_per_color();
        let length = self.data.len();

        if self.current_pix + num_next_bytes + 1 > length {
            None
        } else {
            let end_index = self.current_pix + num_next_bytes;
            let sl = &self.data[self.current_pix..end_index];
            self.fetcher.formatter().get_color(sl).ok()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.data.len() - self.current_pix) / self.fetcher.formatter().bytes_per_color();
        (0, Some(len))
    }
}

impl<'a> ExactSizeIterator for PixelBufferIter<'a> {}

impl PixelBuffer {
    /// Create a new pixel buffer from the specified vector of pixels.
    #[inline]
    pub fn new(data: Vec<u8>, size: Size2D<usize>, format: Format) -> Self {
        let data: Arc<[u8]> = data.into_boxed_slice().into();

        Self {
            data,
            size,
            fetcher: format.into(),
        }
    }

    /// Get the format used to encode the bytes in this image.
    #[inline]
    pub fn format(&self) -> Format {
        self.fetcher.format()
    }

    /// Create an iterator that iterates over the pixels.
    #[inline]
    pub fn iter(&self) -> PixelBufferIter<'_> {
        PixelBufferIter {
            data: &self.data,
            current_pix: 0,
            fetcher: self.fetcher,
        }
    }

    // helper function to get the index of a color in our "array"
    #[inline]
    const fn xy_to_index(&self, x: usize, y: usize) -> usize {
        x + (y * self.size.width)
    }

    /// Get the color at a certain coordinate.
    #[inline]
    pub fn at(&self, pt: Point2D<usize>) -> Option<Color> {
        self.iter().nth(self.xy_to_index(pt.x, pt.y))
    }
}
