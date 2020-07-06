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

use crate::{
    mutexes::{Mutex, RwLock},
    Event, EventHandler, EventType, GenericWindowInternal, Texture, Window,
};
use alloc::{boxed::Box, collections::VecDeque, string::String, sync::Arc};
use core::{mem, option::Option};
use euclid::default::Rect;
use hashbrown::{HashMap, HashSet};
#[cfg(feature = "std")]
use parking_lot::{MappedMutexGuard, MutexGuard};
#[cfg(windows)]
use porcupine::prelude::*;
use smallvec::SmallVec;

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
    #[cfg(windows)]
    next_events: Mutex<SmallVec<[crate::Result<SmallVec<[Event; 2]>>; 1]>>,
}

/// An instance of the Beetle GUI window factory.
///
/// The Instance object is used to abstract over the connection to the GUI server
/// that is needed to create windows and widgets.
#[repr(transparent)]
pub struct Instance(Arc<InstanceInternal>);

impl PartialEq for Instance {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Instance {}

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

    /// Queue an event into the event queue.
    #[inline]
    pub fn queue_event(&self, ev: Event) {
        self.0.event_queue.lock().push_back(ev);
    }

    /// Queue several events into the event queue.
    #[inline]
    pub fn queue_events<I: IntoIterator<Item = Event>>(&self, evs: I) {
        let mut evq = self.0.event_queue.lock();
        evs.into_iter().for_each(|e| evq.push_back(e));
    }

    /// Get the next event.
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
        match evq.pop_front() {
            Some(ev) => Ok(ev),
            None => {
                mem::drop(evq); // hold_for_events might need the mutex

                // hold for the next event
                let mut ne: Option<Event> = None;
                while ne.is_none() {
                    let mut new_evs = hold_for_events(self)?;
                    let mut drain = new_evs
                        .drain(..)
                        .filter(|e| e.window().receives_event(&e.ty()));
                    let mut evq = self.0.event_queue.lock();

                    ne = drain.next();
                    drain.for_each(|ev| evq.push_back(ev));
                }
                Ok(ne.unwrap())
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
    fn flutterbug_new() -> crate::Result<Instance> {
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
    fn flutterbug_add_window(&self, external_id: WindowID, window: &Window) {
        let mut l = self.0.window_mappings.lock();
        l.insert(external_id, window.clone());
    }

    /// Get the display.
    #[inline]
    pub fn display(&self) -> &flutterbug::Display {
        &self.0.connection
    }

    #[inline]
    pub(crate) fn im(&self) -> &flutterbug::InputMethod {
        &self.0.im
    }

    /// Get a window from the window mappings.
    #[inline]
    pub(crate) fn flutterbug_get_window(&self, ex_id: WindowID) -> Option<Window> {
        let l = self.0.window_mappings.lock();
        l.get(&ex_id).map(|w| w.clone())
    }

    #[inline]
    pub(crate) fn delete_window_atom(&self) -> flutterbug::Atom {
        self.0.atoms[DELETE_WINDOW_ATOM]
    }

    #[inline]
    fn flutterbug_create_window(
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
            Arc::new(RwLock::new(cw)),
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
    fn porcupine_new() -> crate::Result<Instance> {
        // win32 doesn't really have a connection object like X11 does
        // however, we do well to initialize CommCtrl here
        porcupine::init_commctrl(porcupine::ControlClasses::BAR_CLASSES)?;

        Ok(Self(Arc::new(InstanceInternal {
            event_queue: Mutex::new(VecDeque::new()),
            window_mappings: Mutex::new(HashMap::new()),
            next_events: Mutex::new(SmallVec::new()),
        })))
    }

    #[inline]
    fn porcupine_create_window(
        &self,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
        is_top_level: bool,
    ) -> crate::Result<Window> {
        let cw = crate::WindowInternal::new(self, parent, text, bounds, background, is_top_level)?;
        let id = cw.id();
        let ex_id = cw.inner_porc_window().hwnd().as_ptr() as *const () as usize;

        let w = Window::from_raw(
            Arc::new(RwLock::new(cw)),
            Arc::new(Mutex::new(HashSet::new())),
            id,
            self.clone(),
            None,
        );

        // add to window mappings
        let mut wm = self.0.window_mappings.lock();
        wm.insert(ex_id, w.clone());
        Ok(w)
    }

    #[inline]
    pub(crate) fn porcupine_get_window(
        &self,
        hwnd: porcupine::winapi::shared::windef::HWND,
    ) -> Option<Window> {
        log::trace!("Accessing window by HWND {:p}", hwnd);
        let wm = self.0.window_mappings.lock();
        wm.get(&(hwnd as *const () as usize)).map(|w| w.clone())
    }

    #[inline]
    pub(crate) fn porcupine_set_next_events(&self, ne: crate::Result<SmallVec<[Event; 2]>>) {
        let mut l = self.0.next_events.lock();
        l.push(ne);
    }

    #[inline]
    pub(crate) fn porcupine_hold_for_events(&self) -> crate::Result<SmallVec<[Event; 2]>> {
        // run a single iteration of the message loop
        if let Some(ref msg) = porcupine::get_message()? {
            porcupine::translate_message(msg);
            porcupine::dispatch_message(msg);
        }

        // drain the next_events variable into the event queue, save for the first element
        let mut next_events = self.0.next_events.lock();
        let mut drain = next_events.drain(..);
        match drain.next() {
            None => Ok(SmallVec::new()), // just return an empty SmallVec. This is just a stack allocation.
            Some(evs) => {
                // if the remaining length is 1 or more, drain it into the event queue
                if drain.len() > 0 {
                    let mut evq = self.0.event_queue.lock();
                    drain.try_for_each::<_, crate::Result<()>>(|nevs| {
                        nevs?.into_iter().for_each(|e| evq.push_back(e));
                        Ok(())
                    })?;
                }

                evs
            }
        }
    }
}
