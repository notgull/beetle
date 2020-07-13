/* -----------------------------------------------------------------------------------
 * src/instance/mod.rs - Instance of the Beetle window factory.
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
    Event, EventData, GenericWindowInternal, Texture, Window,
};
use alloc::{collections::VecDeque, string::String, sync::Arc};
use core::{fmt, mem, option::Option};
use euclid::default::Rect;
use hashbrown::{HashMap, HashSet};
#[cfg(windows)]
use porcupine::prelude::*;
use smallvec::SmallVec;

#[cfg(feature = "expose_internals")]
pub mod internal;
#[cfg(not(feature = "expose_internals"))]
pub(crate) mod internal;

mod loader;

struct InstanceInner {
    event_queue: Mutex<VecDeque<Event>>,
    backend: internal::InternalInstance,
}

/// An instance of the Beetle GUI window factory.
///
/// The Instance object is used to abstract over the connection to the GUI server
/// that is needed to create windows and widgets.
#[repr(transparent)]
pub struct Instance(Arc<InstanceInner>);

impl fmt::Debug for Instance {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Instance")
    }
}

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
    ///
    /// This function initializes any connections with the GUI server in
    /// question, as well as initializes the window map and event queue.
    #[inline]
    pub fn new() -> crate::Result<Instance> {
        // use the loader to dynamically load the internal instance,
        // or any required libraries
        Ok(Self(Arc::new(InstanceInner {
            event_queue: Mutex::new(VecDeque::new()),
            backend: loader::load()?,
        })))
    }

    /// Create a new window. This function initializes the window (or equivalent) in
    /// the backend that the Instance is targeting.
    #[inline]
    pub fn create_window(
        &self,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32>,
        background: Option<Texture>,
    ) -> crate::Result<Window> {
        #[inline]
        fn create_window_np(
            this: &Instance,
            parent: Option<&Window>,
            text: String,
            bounds: Rect<u32>,
            background: Option<Texture>,
        ) -> crate::Result<Window> {
            this.0.backend.generic().create_window(parent, text, bounds, background, this.clone())
        }

        let w = create_window_np(self, parent, text, bounds, background)?;
        w.set_bounds(bounds)?;
        Ok(w)
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
        evq.extend(evs);
    }

    /// Get the next event.
    #[inline]
    pub fn next_event(&self) -> crate::Result<Event> {
        let mut evq = self.0.event_queue.lock(); // option so we can drop it
        loop {
            match evq.pop_front() {
                Some(ev) => return Ok(ev),
                None => {
                    self.0.backend.generic().hold_for_events(&mut evq)?;
                }
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
        l.get(&ex_id).cloned()
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
        let cw = crate::WindowInternal::new(self, parent, text, bounds, background, is_top_level)?;
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
        // hashmap can only store the usize
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
        mem::drop(wm);

        // force a repaint now that it's initialized
        w.repaint(None)?;

        Ok(w)
    }

    #[inline]
    pub(crate) fn porcupine_get_window(
        &self,
        hwnd: porcupine::winapi::shared::windef::HWND,
    ) -> Option<Window> {
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
        let mut drain = next_events.drain(..).rev();
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
