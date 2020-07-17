/* -----------------------------------------------------------------------------------
 * src/instance/internal/flutter.rs - Backend instance for Flutterbug
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

use crate::{
    mutexes::{Mutex, RwLock},
    Event, Instance, Pixel, Texture, Window,
};
use alloc::{collections::VecDeque, string::String};
use euclid::Rect;
use flutterbug::{prelude::*, x11::xlib::Window as WindowID, Atom, Display, InputMethod};
use hashbrown::HashMap;
use smallvec::SmallVec;

const DELETE_WINDOW_ATOM: usize = 0;

pub struct FlutterII {
    connection: Display,
    atoms: [Atom; 1],
    im: InputMethod,
    window_mappings: RwLock<HashMap<WindowID, Window>>,
}

impl FlutterII {
    #[inline]
    pub fn new() -> crate::Result<Self> {
        let dpy = Display::new()?;

        Ok(Self {
            window_mappings: RwLock::new(HashMap::new()),
            atoms: [dpy.internal_atom("WM_DELETE_WINDOW", false)?],
            im: dpy.input_method()?,
            connection: dpy,
        })
    }

    #[inline]
    pub fn delete_window_atom(&self) -> Atom {
        self.atoms[DELETE_WINDOW_ATOM]
    }

    #[inline]
    pub fn input_method(&self) -> &InputMethod {
        &self.im
    }

    #[inline]
    pub fn connection(&self) -> &Display {
        &self.connection
    }

    #[inline]
    pub fn fl_get_window(&self, winid: WindowID) -> Option<Window> {
        match self.window_mappings.try_read() {
            Some(wm) => wm.get(&winid).cloned(),
            None => {
                log::error!("Unable to acquire read access to Flutterbug window mappings",);
                None
            }
        }
    }
}

impl super::GenericInternalInstance for FlutterII {
    #[inline]
    fn create_window(
        &self,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32, Pixel>,
        background: Option<Texture>,
        instance_ref: Instance,
    ) -> crate::Result<Window> {
        let fiw = crate::window::internal::flutter::FlutterIW::new(
            &self.connection,
            &self.im,
            self.delete_window_atom(),
            parent,
            &text,
            bounds,
        )?;
        let ex_id = fiw.fl_window().window();

        let w = Window::from_raw(
            crate::window::InternalWindow::Flutter(fiw),
            Mutex::new(crate::window::WindowProperties::new(
                text,
                bounds,
                background,
                parent.is_none(),
            )),
            instance_ref,
        );

        self.window_mappings.write().insert(ex_id, w.clone());

        Ok(w)
    }

    #[inline]
    fn hold_for_events(
        &self,
        output: &mut SmallVec<[Event; 8]>,
        inst: &Instance,
    ) -> crate::Result<()> {
        output.extend(Event::from_flutter(
            inst,
            flutterbug::Event::next(&self.connection)?,
        )?);
        Ok(())
    }
}
