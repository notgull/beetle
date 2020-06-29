/* -----------------------------------------------------------------------------------
 * src/instance.rs - Instance of the Beetle window factory.
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

use crate::{Event, EventType, GenericWindowInternal, Texture, Window};
use euclid::default::Rect;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use smallvec::SmallVec;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    mem,
    sync::Arc,
};

struct InstanceInternal {
    event_queue: Mutex<VecDeque<Event>>,

    #[cfg(target_os = "linux")]
    window_mappings: Mutex<HashMap<WindowID, Window>>,
    #[cfg(target_os = "linux")]
    connection: flutterbug::Display,
    #[cfg(target_os = "linux")]
    atoms: [flutterbug::Atom; 1],
    #[cfg(target_os = "linux")]
    im: flutterbug::InputMethod,

    #[cfg(windows)]
    window_mappings: Mutex<HashMap<usize, Window>>,
}

/// An instance of the Beetle GUI window factory.
pub struct Instance(Arc<InstanceInternal>);

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

impl Clone for Instance {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Instance {
    /// Create the default instance of the Beetle GUI factory.
    #[inline]
    pub fn new() -> crate::Result<Instance> {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "linux")] {
                Self::flutterbug_new()
            } else if #[cfg(windows)] {
                Self::porcupine_new()
            } else {
                unimplemented!()
            }
        }
    }

    /// Get the next event in the event queue.
    #[inline]
    pub fn next_event(&self) -> crate::Result<Event> {
        #[inline]
        fn hold_for_events(this: &Instance) -> crate::Result<SmallVec<[Event; 2]>> {
            cfg_if::cfg_if! {
                if #[cfg(target_os = "linux")] {
                    Event::from_flutter(this, flutterbug::Event::next(&this.0.connection)?)
                } else if #[cfg(windows)] {
                    this.porcupine_hold_for_events()
                } else {
                    unimplemented!()
                }
            }
        }

        let mut evq = self.0.event_queue.lock();
        while evq.len() == 0 {
            hold_for_events(self)?
                .into_iter()
                .filter(|e| e.window().receives_event(&e.ty())) // filter out events the window can't receive
                .for_each(|e| evq.push_back(e));
        }
        Ok(evq.pop_front().unwrap())
    }

    /// Enqueue an event in the event queue.
    #[inline]
    pub fn queue_event(&self, ev: Event) {
        let mut evq = self.0.event_queue.lock();
        evq.push_back(ev);
    }

    /// Create a new window.
    #[inline]
    pub fn create_window(
        &self,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
        is_top_level: bool,
    ) -> crate::Result<Window> {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "linux")] {
                self.flutterbug_create_window(parent, text, bounds, background, is_top_level)
            } else if #[cfg(windows)] {
                self.porcupine_create_window(parent, text, bounds, background, is_top_level)
            } else {
                unimplemented!()
            }
        }
    }
}

#[cfg(target_os = "linux")]
const DELETE_WINDOW_ATOM: usize = 0;

#[cfg(target_os = "linux")]
use flutterbug::x11::xlib::Window as WindowID;

#[cfg(target_os = "linux")]
impl Instance {
    /// Create the flutterbug instance of the Beetle GUI factory.
    pub fn flutterbug_new() -> crate::Result<Instance> {
        use flutterbug::{prelude::*, Display};

        let dpy = Display::new()?;

        Ok(Self(Arc::new(InstanceInternal {
            event_queue: Mutex::new(VecDeque::new()),
            window_mappings: Mutex::new(HashMap::new()),
            atoms: [dpy.internal_atom("WM_DELETE_WINDOW", false)?],
            im: dpy.input_method()?,
            connection: dpy,
        })))
    }

    /// Add a window.
    #[inline]
    pub fn flutterbug_add_window(&self, external_id: WindowID, window: &Window) {
        let mut l = self.0.window_mappings.lock();
        l.insert(external_id, window.clone());
    }

    /// Get the display.
    #[inline]
    pub fn display(&self) -> &flutterbug::Display {
        &self.0.connection
    }

    #[inline]
    pub fn im(&self) -> &flutterbug::InputMethod {
        &self.0.im
    }

    /// Get a window from the window mappings.
    #[inline]
    pub fn flutterbug_get_window(&self, ex_id: WindowID) -> Option<MappedMutexGuard<'_, Window>> {
        let l = self.0.window_mappings.lock();

        // TODO: streamline this so we only have to access once
        match l.get(&ex_id) {
            Some(_w) => Some(MutexGuard::map(l, move |wm| wm.get_mut(&ex_id).unwrap())),
            None => None,
        }
    }

    #[inline]
    pub fn delete_window_atom(&self) -> flutterbug::Atom {
        self.0.atoms[DELETE_WINDOW_ATOM]
    }

    #[inline]
    pub fn flutterbug_create_window(
        &self,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
        is_top_level: bool,
    ) -> crate::Result<Window> {
        let mut cw =
            crate::WindowInternal::new(self, parent, text, bounds, background, is_top_level)?;
        let id = cw.id();
        let ex_id = cw.inner_flutter_window().window();

        let w = Window::from_raw(
            Arc::new(Mutex::new(cw)),
            Arc::new(Mutex::new(HashSet::new())),
            id,
            self.clone(),
            None,
        );
        self.flutterbug_add_window(ex_id, &w);
        Ok(w)
    }
}

#[cfg(windows)]
use porcupine::HWND;

#[cfg(windows)]
impl Instance {
    #[inline]
    pub fn porcupine_new() -> crate::Result<Instance> {
        // win32 doesn't really have a connection object like X11 does
        // however, we do well to initialize CommCtrl here
        porcupine::init_commctrl(porcupine::ControlClasses::BAR_CLASSES)?;

        Ok(Self(Arc::new(InstanceInternal {
            event_queue: Mutex::new(VecDeque::new()),
            window_mappings: Mutex::new(HashMap::new()),
        })))
    }

    #[inline]
    pub fn porcupine_add_window(&self, external_id: HWND, window: &Window) {
        let hashable_index = external_id as usize;
        let mut l = self.0.window_mappings.lock();
        l.insert(hashable_index, window.clone());
    }

    #[inline]
    pub fn porcupine_get_window(&self, external_id: HWND) -> Option<MappedMutexGuard<'_, Window>> {
        let index = external_id as usize;
        let l = self.0.window_mappings.lock();

        // TODO: only one access, as above
        match l.get(&index) {
            None => None,
            Some(_e) => Some(MutexGuard::map(l, move |i| i.get_mut(&index).unwrap())),
        }
    }

    #[inline]
    pub fn porcupine_create_window(
        &self,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
        is_top_level: bool,
    ) -> crate::Result<Window> {
        let mut cw =
            crate::WindowInternal::new(self, parent, text, bounds, background, is_top_level)?;
        let id = cw.id();
        let ex_id = cw.inner_porc_window().hwnd().as_ptr();

        let w = Window::from_raw(
            Arc::new(Mutex::new(cw)),
            Arc::new(Mutex::new(HashSet::new())),
            id,
            self.clone(),
            None,
        );
        self.porcupine_add_window(ex_id, &w);
        Ok(w)
    }

    #[inline]
    fn porcupine_hold_for_events(&self) -> crate::Result<SmallVec<[Event; 2]>> {
        use smallvec::smallvec;

        // we'll just intercept the event from the event loop
        // NOTE: fix this if it causes problems
        if let Some(msg) = porcupine::get_message()? {
            porcupine::translate_message(&msg);
            Event::from_porc(self, msg)
        } else {
            // if get_message return None, we need to quit
            // any window here should work
            let wm = self.0.window_mappings.lock();
            let any_window = wm
                .iter()
                .map(|(_k, v)| v)
                .next()
                .expect("Did not have any windows to assign a quit event to!");
            let mut quit_ev = Event::new(any_window, EventType::Quit, vec![]);
            quit_ev.set_is_exit_event(true);
            Ok(smallvec![quit_ev])
        }
    }
}
