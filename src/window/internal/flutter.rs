/* -----------------------------------------------------------------------------------
 * src/window/internal/flutter.rs - Backend window for Flutterbug
 * beetle - Pull-based GUI framework
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

use crate::{mutexes::Once, EventType, Pixel, Window};
use core::convert::TryInto;
use euclid::{Point2D, Rect, Size2D};
use flutterbug::{prelude::*, Atom, Display, EventMask, InputContext, InputMethod, Window as FWindow};
use hashbrown::HashMap;

pub struct FlutterIW {
    inner: FWindow,
    ic: InputContext,
    // make a note of if we're top-level or not
    top_level: bool,
}

#[inline]
fn fl_compat_rect(r: Rect<u32, Pixel>) -> crate::Result<(euclid::default::Point2D<i32>, euclid::default::Size2D<u32>)> {
    let pt = Point2D::new(r.origin.x.try_into()?, r.origin.y.try_into()?);
    let sz = Size2D::new(r.size.width, r.size.height);
    Ok((pt, sz))
}

// macro for determining which event masks to apply
macro_rules! default_event_mask {
    ($itl: expr) => {{
        let mut dem = EventMask::EXPOSURE_MASK;

        if $itl {
            dem |= EventMask::RESIZE_REDIRECT_MASK;
        }

        dem
    }};
}

impl FlutterIW {
    // create a new flutterbug window
    #[inline]
    pub fn new(
        dpy: &Display,
        im: &InputMethod,
        dwp: Atom,
        parent: Option<&Window>,
        text: &str,
        bounds: Rect<u32, Pixel>,
    ) -> crate::Result<Self> {
        let (pt, sz) = fl_compat_rect(bounds)?;
        let inner = dpy.create_simple_window(
            match parent {
                Some(w) => Some(w.fl_inner_window().ok_or_else(|| crate::Error::WindowMismatch)?),
                None => None,
            },
            pt,
            sz,
            1,
            dpy.default_white_pixel()?,
            dpy.default_white_pixel()?,
        )?;

        inner.set_protocols(&mut [dwp])?;
        inner.store_name(text)?;
        inner.select_input(default_event_mask!(parent.is_none()))?;
        inner.set_position(pt)?;

        Ok(Self {
            ic: inner.input_context(im)?,
            inner,
            top_level: parent.is_none(),
        })
    }

    #[inline]
    pub fn fl_window(&self) -> &FWindow {
        &self.inner
    }

    #[inline]
    pub fn ic(&self) -> &InputContext {
        &self.ic
    }
}

fn x11_event_mask_map() -> HashMap<EventType, EventMask> {
    let mut emm = HashMap::new();
    emm
}

impl super::GenericInternalWindow for FlutterIW {
    #[inline]
    fn show(&self) -> crate::Result<()> {
        self.inner.map(true)?;
        Ok(())
    }

    #[inline]
    fn set_size(&self, size: Size2D<u32, Pixel>) -> crate::Result<()> {
        self.inner.resize(Size2D::new(size.width, size.height))?;
        Ok(())
    }

    #[inline]
    fn set_text(&self, text: &str) -> crate::Result<()> {
        self.inner.store_name(text)?;
        Ok(())
    }

    #[inline]
    fn receive_events(&self, events: &[EventType]) -> crate::Result<()> {
        lazy_static::lazy_static! {
            static ref X11_EVENT_MASK_MAP: HashMap<EventType, EventMask> = x11_event_mask_map();
        }

        Ok(self.inner.select_input(
            events
                .iter()
                .map(|et| X11_EVENT_MASK_MAP.get(et).copied())
                .filter(Option::is_some)
                .map(|o| o.unwrap())
                .fold(default_event_mask!(self.top_level), |res, em| res | em),
        )?)
    }
}
