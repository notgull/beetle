/* -----------------------------------------------------------------------------------
 * src/event/flutter.rs - Translate a Beetle Event to a Flutterbug one.
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

use super::{Event, EventData};
use crate::{Graphics, Instance, KeyInfo, KeyType, MouseButton, Window};
use core::convert::TryInto;
use euclid::default::Point2D;
use flutterbug::{prelude::*, Atom, Event as FEvent, EventType as FEventType, FunctionKeys};
use smallvec::SmallVec;

impl Event {
    /// Translate a Flutterbug event to a Beetle event.
    pub(crate) fn from_flutter(
        instance: &Instance,
        fev: FEvent,
    ) -> crate::Result<SmallVec<[Self; 2]>> {
        // optimize for at least two events
        // TODO: this can probably be a TinyVec, if we want to go that route
        let mut evs = SmallVec::new();
        let ty = fev.kind();
        let assoc_window: Window = match instance.flutterbug_get_window(fev.window()) {
            Some(w) => w,
            None => {
                // we don't care about this event, just return nothing
                log::warn!("Found event without a corresponding window: {:?}", fev);
                return Ok(evs);
            }
        };

        log::debug!("Translating Flutterbug Event: {:?}", &fev);

        match fev {
            // X11 events involving a key press
            FEvent::Key(k) => {
                // get the key information from the event
                let (ks, _char_rep) = k.lookup_utf8(&*assoc_window.inner_window()?.ic())?;
                let mut ki = KeyInfo::new(KeyType::from_keysym(
                    ks.ok_or_else(|| crate::Error::KeysymNotFound)?,
                ));

                // set function key info
                #[inline]
                fn fn_key_info<F>(
                    ki: &mut KeyInfo,
                    k: &flutterbug::KeyEvent,
                    key: FunctionKeys,
                    setter: F,
                ) where
                    F: FnOnce(&mut KeyInfo),
                {
                    if k.has_function(key) {
                        setter(ki);
                    }
                }

                fn_key_info(&mut ki, &k, FunctionKeys::CONTROL, |k| k.set_ctrl(true));
                fn_key_info(&mut ki, &k, FunctionKeys::ALT, |k| k.set_alt(true));
                fn_key_info(&mut ki, &k, FunctionKeys::SHIFT, |k| k.set_shift(true));

                // key press mouse location
                let loc: Option<Point2D<u32>> =
                    if let (Ok(x), Ok(y)) = (k.x().try_into(), k.y().try_into()) {
                        Some(Point2D::new(x, y))
                    } else {
                        None
                    };

                evs.push(Event::new(
                    &assoc_window,
                    match ty {
                        FEventType::KeyPress => EventData::KeyDown(ki, loc),
                        FEventType::KeyRelease => EventData::KeyUp(ki, loc),
                        _ => unreachable!(),
                    },
                ));
            }
            // Re-rendering of the window
            FEvent::Expose(e) => {
                // if the size isn't the same, set up a changed bounds event
                let old_bounds = assoc_window.bounds()?;
                let new_bounds =
                    euclid::rect(e.x().try_into()?, e.y().try_into()?, e.width(), e.height());

                if old_bounds != new_bounds {
                    // Note: The (false, true) at the end tells the event handler that
                    // this event was emitted by the event loop and not from the
                    // set_bounds function. It signals that the bounds change should
                    // not be forwarded to the X11 backend. The "true" is to tell the
                    // handler to enqueue the BoundsChanged event.
                    let mut ev = Event::new(
                        &assoc_window,
                        EventData::BoundsChanging {
                            old: old_bounds,
                            new: new_bounds,
                        },
                    );
                    ev.set_hidden_data((false, true));
                    evs.push(ev);
                }

                evs.push(Event::new(
                    &assoc_window,
                    EventData::Paint(Graphics::from_window(&assoc_window)?),
                ));
                // TODO: create g-object
            }
            // Press/release of a mouse button
            #[allow(non_upper_case_globals)]
            FEvent::Button(b) => {
                use flutterbug::x11::xlib::{Button1, Button2, Button3, Button4, Button5};
                if let (Ok(x), Ok(y)) = (b.x().try_into(), b.y().try_into()) {
                    let button = match b.button() {
                        Button1 => MouseButton::Button1,
                        Button2 => MouseButton::Button2,
                        Button3 => MouseButton::Button3,
                        Button4 => MouseButton::Button4,
                        Button5 => MouseButton::Button5,
                        _ => return Err(crate::Error::StaticMsg("Unexpected X11 mouse input")),
                    };
                    let loc = Point2D::<u32>::new(x, y);

                    evs.push(Event::new(
                        &assoc_window,
                        match b.kind() {
                            FEventType::ButtonPress => EventData::MouseButtonDown(loc, button),
                            FEventType::ButtonRelease => EventData::MouseButtonUp(loc, button),
                            _ => unreachable!(),
                            // First element is the X/Y coordinates. Second is the mouse button pressed.
                        },
                    ));
                }
            }
            // Special client messages
            FEvent::ClientMessage(c) => {
                // Check if the client message corresponds to the pre-set delete window atom
                if AsRef::<[Atom]>::as_ref(&c.data())[0] == instance.delete_window_atom() {
                    evs.push(Event::new(&assoc_window, EventData::Close));

                    // also send a quit event if this is the top-level window
                    if assoc_window.is_top_level()? {
                        let mut quit_ev = Event::new(&assoc_window, EventData::Quit);
                        quit_ev.set_is_exit_event(true);
                        evs.push(quit_ev);
                    }
                }
            }
            _ => { /* TODO: don't ignore these! */ }
        }

        Ok(evs)
    }
}
