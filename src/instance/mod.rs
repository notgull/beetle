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
    Event, EventData, EventLoop, Pixel, Texture, Window,
};
use alloc::{collections::VecDeque, string::String, sync::Arc};
use core::{fmt, mem, option::Option};
use euclid::Rect;
use hashbrown::{HashMap, HashSet};
#[cfg(windows)]
use porcupine::prelude::*;
use smallvec::SmallVec;

#[cfg(feature = "expose_internals")]
pub mod internal;
#[cfg(not(feature = "expose_internals"))]
pub(crate) mod internal;

use internal::*;

mod loaded;
mod loader;
pub use loaded::*;

struct InstanceInner<T: EventLoop + ?Sized> {
    event_queue: Mutex<VecDeque<Event>>,
    event_loop: Mutex<Option<T>>,
    backend: internal::InternalInstance,
}

/// An instance of the Beetle GUI window factory.
///
/// The Instance object is used to abstract over the connection to the GUI server
/// that is needed to create windows and widgets.
#[repr(transparent)]
pub struct Instance<T: EventLoop + ?Sized>(Arc<InstanceInner<T>>);

impl<T: EventLoop + ?Sized> fmt::Debug for Instance<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Instance")
    }
}

impl<T: EventLoop + ?Sized> PartialEq for Instance<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: EventLoop + ?Sized> Eq for Instance<T> {}

unsafe impl<T: EventLoop + ?Sized> Send for Instance<T> {}
unsafe impl<T: EventLoop + ?Sized> Sync for Instance<T> {}

impl<T: EventLoop + ?Sized> Clone for Instance<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: EventLoop + ?Sized + Send> Instance<T> {
    /// Create the default instance of the Beetle GUI factory.
    ///
    /// This function initializes any connections with the GUI server in
    /// question, as well as initializes the window map and event queue.
    #[inline]
    pub fn new() -> crate::Result<Self> {
        // use the loader to dynamically load the internal instance,
        // or any required libraries
        Ok(Self(Arc::new(InstanceInner {
            event_queue: Mutex::new(VecDeque::new()),
            backend: loader::load()?,
        })))
    }

    /// Get the type of library loaded by this Instance.
    #[inline]
    pub fn ty(&self) -> InstanceType {
        self.0.backend.ty()
    }

    /// Create a new window. This function initializes the window (or equivalent) in
    /// the backend that the Instance is targeting.
    #[inline]
    pub fn create_window(
        &self,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32, Pixel>,
        background: Option<Texture>,
    ) -> crate::Result<Window> {
        #[inline]
        fn create_window_np(
            this: &Instance,
            parent: Option<&Window>,
            text: String,
            bounds: Rect<u32, Pixel>,
            background: Option<Texture>,
        ) -> crate::Result<Window> {
            this.0
                .backend
                .generic()
                .create_window(parent, text, bounds, background, this.clone())
        }

        let w = create_window_np(self, parent, text, bounds, background)?;
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
}

#[cfg(target_os = "linux")]
const DELETE_WINDOW_ATOM: usize = 0;

#[cfg(target_os = "linux")]
use flutterbug::x11::xlib::Window as WindowID;

#[cfg(target_os = "linux")]
impl<T: EventLoop + ?Sized> Instance<T> {
    /// Get the display.
    #[inline]
    pub fn display(&self) -> Option<&flutterbug::Display> {
        match self.0.backend {
            InternalInstance::Flutter(ref f) => Some(f.connection()),
            _ => None,
        }
    }

    /// Get a window by its window id.
    #[inline]
    pub(crate) fn flutterbug_get_window(&self, ex_id: flutterbug::x11::xlib::Window) -> Option<Window> {
        match self.0.backend {
            InternalInstance::Flutter(ref f) => f.fl_get_window(ex_id),
            _ => None,
        }
    }

    /// Get the delete window atom.
    #[inline]
    pub(crate) fn delete_window_atom(&self) -> Option<flutterbug::Atom> {
        match self.0.backend {
            InternalInstance::Flutter(ref f) => Some(f.delete_window_atom()),
            _ => None,
        }
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
    pub(crate) fn porcupine_get_window(&self, hwnd: porcupine::winapi::shared::windef::HWND) -> Option<Window> {
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
