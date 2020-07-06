/* -----------------------------------------------------------------------------------
 * src/window/internal/porc.rs
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

use super::{super::Window, unique_id, EventHandler, GenericWindowInternal};
use crate::{take_vec::TakeVec, Event, EventType, Instance, Texture};
use alloc::string::{String, ToString};
use euclid::default::Rect;
use porcupine::{
    prelude::*,
    winapi::{
        shared::{
            minwindef::{LPARAM, UINT, WPARAM},
            windef::HWND,
        },
        um::winuser,
    },
    CmdShow, ExtendedWindowStyle, OwnedWindowClass, Window as PWindow, WindowClass, WindowStyle,
};
use std::{
    any::Any,
    boxed::Box,
    convert::TryInto,
    mem,
    os::raw::c_int,
    sync::{atomic::AtomicPtr, Arc},
};

// a window class that every instance of WindowInternal uses
lazy_static::lazy_static! {
    static ref BEETLE_WINDOW_CLASS: OwnedWindowClass = beetle_window_class()
        .unwrap_or_else(|e| panic!("Unable to create basic Beetle window class: {}", e));
}

fn beetle_window_class() -> crate::Result<OwnedWindowClass> {
    let wc_name = "NotASeagullBeetleWindow".to_string(); // should be unique enough
    let mut wc = OwnedWindowClass::new(wc_name);
    wc.set_window_proc(Some(crate::wndproc::beetle_wndproc));
    wc.register()?;
    Ok(wc)
}

impl WindowClass for BEETLE_WINDOW_CLASS {
    fn identifier(&self) -> &str {
        OwnedWindowClass::identifier(self)
    }
}

pub struct WindowInternal {
    id: usize,
    inner: PWindow,
    event_handler: Box<dyn EventHandler>,
    text: String,
    background: Option<Texture>,
    top_level: bool,
    bounds: Rect<u32>,

    // storage for old bounds for size change events
    old_bounds: TakeVec<Rect<u32>>,
}

impl GenericWindowInternal for WindowInternal {
    #[inline]
    fn id(&self) -> usize {
        self.id
    }

    fn new(
        instance: &Instance,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
        is_top_level: bool,
    ) -> crate::Result<Self> {
        let default_ws = WindowStyle::CLIP_CHILDREN
            | WindowStyle::SYSMENU
            | WindowStyle::SIZEBOX
            | WindowStyle::MINIMIZE_BOX
            | WindowStyle::CAPTION;

        // create internal window
        let mut pw = PWindow::with_creation_param(
            &BEETLE_WINDOW_CLASS,
            &text,
            default_ws,
            ExtendedWindowStyle::CLIENT_EDGE,
            euclid::rect(
                bounds.origin.x.try_into()?,
                bounds.origin.y.try_into()?,
                bounds.size.width.try_into()?,
                bounds.size.height.try_into()?,
            ),
            parent.map(|p| p.inner_porc_window()).as_deref(),
            Some(Box::new(instance.clone())),
        )?;

        Ok(Self {
            inner: pw,
            id: unique_id(),
            event_handler: Box::new(super::default_event_handler),
            text,
            background,
            top_level: is_top_level,
            bounds,
            old_bounds: TakeVec::new(),
        })
    }

    #[inline]
    fn receive_events(&mut self, _events: &[EventType]) -> crate::Result<()> {
        // no-op
        Ok(())
    }

    #[inline]
    fn event_handler(&self) -> &dyn EventHandler {
        &self.event_handler
    }

    #[inline]
    fn set_event_handler<F: EventHandler>(&mut self, evh: F) {
        self.event_handler = Box::new(evh);
    }

    #[inline]
    fn text(&mut self) -> &mut str {
        &mut self.text
    }

    #[inline]
    fn set_text(&mut self, txt: String) -> crate::Result<String> {
        self.inner.set_text(&txt)?;

        let mut res = txt;
        mem::swap(&mut self.text, &mut res);
        Ok(res)
    }

    #[inline]
    fn background(&mut self) -> Option<&mut Texture> {
        self.background.as_mut()
    }

    #[inline]
    fn set_background(&mut self, bg: Option<Texture>) {
        self.background = bg;
    }

    #[inline]
    fn take_background(&mut self) -> Option<Texture> {
        self.background.take()
    }

    #[inline]
    fn bounds(&self) -> Rect<u32> {
        self.bounds
    }

    #[inline]
    fn set_bounds(&mut self, bounds: Rect<u32>, backend: bool) -> crate::Result<Rect<u32>> {
        if backend {
            self.inner.reshape(euclid::rect(
                bounds.origin.x.try_into()?,
                bounds.origin.y.try_into()?,
                bounds.size.width.try_into()?,
                bounds.size.height.try_into()?,
            ))?;
        }

        let mut res = bounds;
        mem::swap(&mut self.bounds, &mut res);
        Ok(res)
    }

    #[inline]
    fn is_top_level(&self) -> bool {
        self.top_level
    }

    #[inline]
    fn show(&self) -> crate::Result<()> {
        // on win32, show() is handled by the Window object
        unimplemented!()
    }

    #[inline]
    fn repaint(&self, bounds: Option<Rect<u32>>) -> crate::Result<()> {
        self.inner.invalidate(bounds.map(|b| {
            euclid::rect(
                b.origin.x as c_int,
                b.origin.y as c_int,
                b.size.width as c_int,
                b.size.height as c_int,
            )
        }))?;
        Ok(())
    }
}

impl WindowInternal {
    #[inline]
    pub(crate) fn inner_porc_window(&mut self) -> &mut PWindow {
        &mut self.inner
    }

    #[inline]
    pub(crate) fn set_user_data<T>(&mut self, obj: T) -> crate::Result<()> {
        let boxed = Box::new(obj);
        Ok(self.inner.set_user_data_box(boxed)?)
    }

    #[inline]
    pub(crate) fn store_old_bounds(&mut self) {
        self.old_bounds.push(self.bounds);
    }

    #[inline]
    pub(crate) fn take_old_bounds(&mut self) -> Option<Rect<u32>> {
        self.old_bounds.take()
    }
}
