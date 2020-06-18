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

use super::{Event, EventType};
use crate::{Instance, KeyInfo, KeyType};
use flutterbug::{prelude::*, Event as FEvent, EventType as FEventType};
use std::sync::Arc;

impl Event {
    /// Translate a Flutterbug event to a Beetle event.
    pub fn from_flutter(instance: &Instance, fev: FEvent) -> crate::Result<Vec<Self>> {
        // optimize for at least one event
        let mut evs = Vec::with_capacity(1);
        let ty = fev.kind();
        let assoc_window = instance
            .get_window(fev.window())
            .ok_or_else(|| crate::Error::WindowNotFound)?;

        match fev {
            FEvent::Key(k) => {
                // get the key information from the event
                let (ks, _char_rep) = k.lookup_utf8(&*assoc_window.ic())?;
                let mut ki = KeyInfo::new(KeyType::from_keysym(
                    ks.ok_or_else(|| crate::Error::KeysymNotFound)?,
                ));

                evs.push(Event::new(
                    &assoc_window,
                    match ty {
                        FEventType::KeyPress => EventType::KeyDown,
                        FEventType::KeyRelease => EventType::KeyUp,
                        _ => unreachable!(),
                    },
                    vec![Arc::new(ki)],
                ));
            }
            _ => { /* TODO: don't ignore these! */ }
        }

        Ok(evs)
    }
}
