/* -----------------------------------------------------------------------------------
 * src/instance/flutter.rs - An instance based on the Flutterbug library.
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

use crate::{Event, Window};
use flutterbug::{
    prelude::*, x11::xlib::Window as WindowID, Display, Event as FEvent, InputContext, InputMethod,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

static EV_QUEUE_MUTEX_FAIL: &'static str = "Unable to achieve lock on event queue";
static WINDOW_MAPPING_MUTEX_FAIL: &'static str =
    "Unable to achieve lock on window ID to window mappings";

pub struct Instance {
    event_queue: Mutex<VecDeque<Event>>,
    window_mappings: Mutex<HashMap<WindowID, Window>>,
    display: Display,
    input_method: InputMethod,
}

impl super::GuiFactory for Instance {
    fn new() -> crate::Result<Self> {
        let display = Display::new()?;

        Ok(Self {
            event_queue: Mutex::new(VecDeque::new()),
            window_mappings: Mutex::new(HashMap::new()),
            input_method: display.input_method()?,
            display,
        })
    }

    fn next_event(&self) -> crate::Result<Event> {
        let mut eq = self.event_queue.lock().expect(EV_QUEUE_MUTEX_FAIL);
        if eq.len() == 0 {
            // get the next X11 event
            let fev = FEvent::next(&self.display)?; // this will hang the current thread
                                                    // until a new event arrives
            Event::from_flutter(self, fev)?
                .into_iter()
                .for_each(|e| eq.push_back(e));
        }
        Ok(eq.pop_front().unwrap()) // this should never panic
    }

    fn queue_event(&self, ev: Event) {
        let mut eq = self.event_queue.lock().expect(EV_QUEUE_MUTEX_FAIL);
        eq.push_back(ev);
    }
}

impl Instance {
    #[inline]
    pub(crate) fn display(&self) -> &Display {
        &self.display
    }

    #[inline]
    pub(crate) fn im(&self) -> &InputMethod {
        &self.input_method
    }

    #[inline]
    pub(crate) fn get_window(&self, id: WindowID) -> Option<Window> {
        let l = self
            .window_mappings
            .lock()
            .expect(WINDOW_MAPPING_MUTEX_FAIL);
        l.get(&id).map(|w| w.clone())
    }

    #[inline]
    pub(crate) fn add_window(&self, id: WindowID, win: Window) {
        let mut l = self
            .window_mappings
            .lock()
            .expect(WINDOW_MAPPING_MUTEX_FAIL);
        l.insert(id, win);
    }
}
