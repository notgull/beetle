/* -----------------------------------------------------------------------------------
 * src/ev_loop.rs - The event loop object the instance uses to process events.
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

use crate::{Event, Instance, Window};
use alloc::{format, vec::Vec};
use core::fmt;

/// Whether or not the program should continue.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EventLoopAction {
    Continue,
    Stop,
}

/// The event loop that the program uses to continually run.
pub trait EventLoop: Sync {
    /// Runs once for each window in the program.
    fn register_window(&mut self, window: &Window, instance: &Instance) -> crate::Result<()>;
    /// Runs once for each event, prior to being dispatched and handled by the windows.
    fn pre_dispatch(&self, event: &Event, instance: &Instance) -> crate::Result<()>;
    /// Runs once for each event, after the event is dispatched and handled.
    fn post_dispatch(&self, event: Event, instance: &Instance) -> crate::Result<()>;
    /// Handle several events.
    #[inline]
    fn handle_events<F, I: IntoIterator<Item = Event>>(
        &self,
        events: I,
        instance: &Instance,
        handler: F,
    ) -> crate::Result<EventLoopAction> where F: Fn(&Self, I, Instance) -> crate::Result<EventLoopAction> {
        handle_events_impl(self, events, instance).map(|v| {
            if v.contains(&EventLoopAction::Stop) {
                EventLoopAction::Stop
            } else {
                EventLoopAction::Continue
            }
        })
    }
}

#[inline]
fn handle_events_impl<T: EventLoop + ?Sized, I>(
    evl: &T,
    events: I,
    instance: &Instance,
) -> crate::Result<Vec<EventLoopAction>>
where
    I: IntoIterator<Item = Event>,
{
    events.into_iter().map(|ev| evl.handle_event_dispatch(ev, instance)).collect()
}

impl<F, E> EventLoop for F
where
    F: Fn(&Event, &Instance) -> core::result::Result<EventLoopAction, E> + Sync,
    E: fmt::Display,
{
    #[inline]
    fn register_window(&mut self, _w: &Window, _i: &Instance) -> crate::Result<()> {
        Ok(())
    }

    #[inline]
    fn handle_event(&self, event: &Event, instance: &Instance) -> crate::Result<EventLoopAction> {
        trait IntoBeetleError {
            fn into_berror(self) -> crate::Error;
        }

        impl<T: fmt::Display> IntoBeetleError for T {
            #[inline]
            default fn into_berror(self) -> crate::Error {
                crate::Error::Msg(format!("{}", &self))
            }
        }

        impl<T: fmt::Display + Into<crate::Error>> IntoBeetleError for T {
            #[inline]
            fn into_berror(self) -> crate::Error {
                self.into()
            }
        }

        match self(event, instance) {
            Ok(eva) => Ok(eva),
            Err(e) => Err(IntoBeetleError::into_berror(e)),
        }
    }
}
