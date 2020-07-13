/* -----------------------------------------------------------------------------------
 * src/instance/internal/porc.rs - Backend instance for Porcupine
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
use hashbrown::HashMap;
use porcupine::winapi::HWND;
use smallvec::{smallvec, SmallVec};

pub struct PorcII {
    window_mappings: RwLock<HashMap<usize, Window>>,
    next_events: Mutex<SmallVec<[crate::Result<SmallVec<[Event; 2]>>; 1]>>,
}

impl PorcII {
    #[inline]
    pub fn new() -> crate::Result<Self> {
        // win32 doesn't really have a connection object like X11 does
        // however, we do well to initialize CommCtrl here
        porcupine::init_commctrl(porcupine::ControlClasses::BAR_CLASSES)?;

        Ok(Self(Arc::new(InstanceInternal {
            window_mappings: RwLock::new(HashMap::new()),
            next_events: Mutex::new(SmallVec::new()),
        })))
    }

    #[inline]
    pub fn pc_get_window(&self, ptr: HWND) -> Option<Window> {
        match self.window_mappings.try_read() {
            Ok(wm) => {
                let ex_id = ptr as *const () as usize;
                wm.get(&ex_id).cloned()
            }
            Err(e) => {
                log::error!(
                    "Unable to acquire read access to Porcupine window mappings: {}",
                    e
                );
                None
            }
        }
    }

    #[inline]
    pub fn pc_set_next_events(&self, ne: crate::Result<SmallVec<[Event; 2]>>) {
        let mut l = self.next_events.lock();
        l.push(ne);
    }
}

impl super::GenericInternalInstance for PorcII {
    #[inline]
    fn create_window(
        &self,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32, Pixel>,
        background: Option<Texture>,
        instance_ref: Instance,
    ) -> crate::Result<Window> {
        let piw = crate::window::PorcIW::new(parent, &text, bounds)?;
        Ok(Window::from_raw(
            RwLock::new(crate::window::InternalWindow::Porc(piw)),
            Mutex::new(crate::window::WindowProperties::new(
                text, bounds, background,
            )),
            instance_ref,
        ))
    }

    #[inline]
    fn hold_for_events(&self, output: &mut VecDeque<Event>) -> crate::Result<()> {
        let mut ne = self.next_events.lock();
        output.extend(
            ne.drain(..)
                .flat_map(|el| match el {
                    Ok(e) => e.into_iter().map(Result::Ok),
                    Err(err) => smallvec![Err(err)].into_iter(),
                })
                .collect::<crate::Result<SmallVec<[Event; 2]>>>()?,
        );
        Ok(())
    }
}
