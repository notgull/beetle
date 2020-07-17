/* -----------------------------------------------------------------------------------
 * src/window/internal/porc.rs - Backend window for Porcupine
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

use crate::{EventType, Instance, Pixel, Window};
use alloc::string::ToString;
use core::convert::TryInto;
use cty::c_int;
use euclid::{Rect, Size2D};
use porcupine::{
    prelude::*, CmdShow, ExtendedWindowStyle, OwnedWindowClass, Window as PWindow, WindowStyle,
};

#[repr(transparent)]
pub struct PorcIW {
    inner: PWindow,
}

// the window class, statically created and held
lazy_static::lazy_static! {
    static ref BEETLE_WINDOW_CLASS: OwnedWindowClass =
        create_window_class()
            .expect("Unable to create window class");
}

// generator for creating the window class
#[inline]
fn create_window_class() -> crate::Result<OwnedWindowClass> {
    const WC_NAME: &'static str = "BeetleWindowClassDefault";
    let mut wc = OwnedWindowClass::new(WC_NAME.to_string());
    wc.set_window_proc(Some(crate::wndproc::beetle_wndproc));
    wc.register()?;
    Ok(wc)
}

#[inline]
fn porc_compat_rect(r: Rect<u32, Pixel>) -> crate::Result<euclid::default::Rect<c_int>> {
    Ok(euclid::rect(
        r.origin.x.try_into()?,
        r.origin.y.try_into()?,
        r.size.width.try_into()?,
        r.size.height.try_into()?,
    ))
}

#[inline]
fn porc_compat_size(size: Size2D<u32, Pixel>) -> crate::Result<euclid::defaukt::Size2D<c_int>> {
    Ok(euclid::size2(
        size.width.try_into()?,
        size.height.try_into()?,
    ))
}

impl PorcIW {
    #[inline]
    pub fn new(
        instance: &Instance,
        parent: Option<&Window>,
        text: &str,
        bounds: Rect<u32, Pixel>,
    ) -> crate::Result<Self> {
        Ok(Self {
            inner: {
                PWindow::with_creation_param(
                    &*BEETLE_WINDOW_CLASS,
                    text,
                    WindowStyle::MAXIMIZE_BOX | WindowStyle::MINIMIZE_BOX,
                    ExtendedWindowStyle::CLIENT_EDGE,
                    porc_compat_rect(bounds)?,
                    match parent {
                        None => None,
                        Some(w) => Some(w.prc_inner_window().unwrap()),
                    },
                    Some(Box::new(instance.clone())),
                )?
            },
        })
    }

    #[inline]
    pub fn prc_window(&self) -> &PWindow {
        &self.inner
    }
}

impl super::GenericInternalWindow for PorcIW {
    #[inline]
    fn show(&self) -> crate::Result<()> {
        self.inner.show(CmdShow::Show);
        self.inner.update()?;
        Ok(())
    }

    #[inline]
    fn set_size(&self, bounds: Size2D<u32, Pixel>) -> crate::Result<()> {
        self.inner.resize(porc_compat_size(bounds)?)?;
        Ok(())
    }

    #[inline]
    fn set_text(&self, text: &str) -> crate::Result<()> {
        self.inner.set_text(text)?;
        Ok(())
    }

    #[inline]
    fn receive_events(&self, _events: &[EventType]) -> crate::Result<()> {
        // do nothing
        Ok(())
    }
}
