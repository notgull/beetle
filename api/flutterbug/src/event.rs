/* -----------------------------------------------------------------------------------
 * api/flutterbug/src/event.rs - An event that is emitted by the X11 server. The main
 *                               Event object should be an enum that encompasses all
 *                               other events.
 * beetle - Simple graphics framework for Rust
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

use super::FlutterbugError;
use std::{
    convert::TryFrom,
    os::raw::{c_char, c_int, c_long, c_uint},
};
use x11::xlib;

/*
bitflags::bitflags! {
    #[doc = "The distinct types that an event can have"]
    pub struct EventType : c_int {
        const KEY_PRESS = xlib::KeyPress;
        const KEY_RELEASE = xlib::KeyRelease;
        const BUTTON_PRESS = xlib::ButtonPress;
        const BUTTON_RELEASE = xlib::ButtonRelease;
        const MOTION_NOTIFY = xlib::MotionNotify;
        const ENTER_NOTIFY = xlib::EnterNotify;
        const LEAVE_NOTIFY = xlib::LeaveNotify;
        const FOCUS_IN = xlib::FocusIn;
        const FOCUS_OUT = xlib::FocusOut;
        const KEYMAP_NOTIFY = xlib::KeymapNotify;
        const EXPOSE = xlib::Expose;
        const GRAPHICS_EXPOSE = xlib::GraphicsExpose;
        const NO_EXPOSE = xlib::NoExpose;
        const CIRCULATE_REQUEST = xlib::CirculateRequest;
        const CONFIGURE_REQUEST = xlib::ConfigureRequest;
        const MAP_REQUEST = xlib::MapRequest;
        const RESIZE_REQUEST = xlib::ResizeRequest;
        const CIRCULATE_NOTIFY = xlib::CirculateNotify;
        const CONFIGURE_NOTIFY = xlib::ConfigureNotify;
        const CREATE_NOTIFY = xlib::CreateNotify;
        const DESTROY_NOTIFY = xlib::DestroyNotify;
        const GRAVITY_NOTIFY = xlib::GravityNotify;
        const MAP_NOTIFY = xlib::MapNotify;
        const MAPPING_NOTIFY = xlib::MappingNotify;
        const REPARENT_NOTIFY = xlib::ReparentNotify;
        const UNMAP_NOTIFY = xlib::UnmapNotify;
        const VISIBLITY_NOTIFY = xlib::VisibilityNotify;
        const COLORMAP_EVENT = xlib::ColormapEvent;
        const CLIENT_MESSAGE = xlib::ClientMessage;
        const PROPERTY_NOTIFY = xlib::PropertyNotify;
        const SELECTION_CLEAR = xlib::SelectionClear;
        const SELECTION_NOTIFY = xlib::SelectionNotify;
        const SELECTION_REQUEST = xlib::SelectionRequest;
    }
}
*/

/// The type of an X11 event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelease,
    MotionNotify,
    EnterNotify,
    LeaveNotify,
    FocusIn,
    FocusOut,
    KeymapNotify,
    Expose,
    GraphicsExpose,
    NoExpose,
    CirculateRequest,
    ConfigureRequest,
    MapRequest,
    ResizeRequest,
    CirculateNotify,
    ConfigureNotify,
    CreateNotify,
    DestroyNotify,
    GravityNotify,
    MapNotify,
    MappingNotify,
    ReparentNotify,
    UnmapNotify,
    VisibilityNotify,
    ColormapEvent,
    ClientMessage,
    PropertyNotify,
    SelectionClear,
    SelectionNotify,
    SelectionRequest,
}

impl EventType {
    /// Convert a C integer representing an event to an event
    pub fn from_int(t: c_int) -> Option<Self> {
        Some(match t {
            xlib::KeyPress => Self::KeyPress,
            xlib::KeyRelease => Self::KeyRelease,
            xlib::ButtonPress => Self::ButtonPress,
            xlib::ButtonRelease => Self::ButtonRelease,
            xlib::MotionNotify => Self::MotionNotify,
            xlib::EnterNotify => Self::EnterNotify,
            xlib::LeaveNotify => Self::LeaveNotify,
            xlib::FocusIn => Self::FocusIn,
            xlib::FocusOut => Self::FocusOut,
            xlib::KeymapNotify => Self::KeymapNotify,
            xlib::Expose => Self::Expose,
            xlib::GraphicsExpose => Self::GraphicsExpose,
            xlib::NoExpose => Self::NoExpose,
            xlib::CirculateRequest => Self::CirculateRequest,
            xlib::ConfigureRequest => Self::ConfigureRequest,
            xlib::MapRequest => Self::MapRequest,
            xlib::ResizeRequest => Self::ResizeRequest,
            xlib::CirculateNotify => Self::CirculateNotify,
            xlib::ConfigureNotify => Self::ConfigureNotify,
            xlib::CreateNotify => Self::CreateNotify,
            xlib::DestroyNotify => Self::DestroyNotify,
            xlib::GravityNotify => Self::GravityNotify,
            xlib::MapNotify => Self::MapNotify,
            xlib::MappingNotify => Self::MappingNotify,
            xlib::ReparentNotify => Self::ReparentNotify,
            xlib::UnmapNotify => Self::UnmapNotify,
            xlib::VisibilityNotify => Self::VisibilityNotify,
            xlib::ColormapNotify => Self::ColormapEvent,
            xlib::ClientMessage => Self::ClientMessage,
            xlib::PropertyNotify => Self::PropertyNotify,
            xlib::SelectionClear => Self::SelectionClear,
            xlib::SelectionNotify => Self::SelectionNotify,
            xlib::SelectionRequest => Self::SelectionRequest,
            _ => return None,
        })
    }
}

bitflags::bitflags! {
    #[doc = "The masks that can be applied to an event listener"]
    pub struct EventMask : c_long {
        const NO_EVENT_MASK = xlib::NoEventMask;
        const KEY_PRESS_MASK = xlib::KeyPressMask;
        const KEY_RELEASE_MASK = xlib::KeyReleaseMask;
        const BUTTON_PRESS_MASK = xlib::ButtonPressMask;
        const BUTTON_RELEASE_MASK = xlib::ButtonReleaseMask;
        const ENTER_WINDOW_MASK = xlib::EnterWindowMask;
        const LEAVE_WINDOW_MASK = xlib::LeaveWindowMask;
        const POINTER_MOTION_MASK = xlib::PointerMotionMask;
        const POINTER_MOTION_HINT_MASK = xlib::PointerMotionHintMask;
        const BUTTON_1_MOTION_MASK = xlib::Button1MotionMask;
        const BUTTON_2_MOTION_MASK = xlib::Button2MotionMask;
        const BUTTON_3_MOTION_MASK = xlib::Button3MotionMask;
        const BUTTON_4_MOTION_MASK = xlib::Button4MotionMask;
        const BUTTON_5_MOTION_MASK = xlib::Button5MotionMask;
        const BUTTON_MOTION_MASK = xlib::ButtonMotionMask;
        const KEYMAP_STATE_MASK = xlib::KeymapStateMask;
        const EXPOSURE_MASK = xlib::ExposureMask;
        const VISIBILITY_CHANGE_MASK = xlib::VisibilityChangeMask;
        const STRUCTURE_NOTIFY_MASK = xlib::StructureNotifyMask;
        const RESIZE_REDIRECT_MASK = xlib::ResizeRedirectMask;
        const SUBSTRUCTURE_NOTIFY_MASK = xlib::SubstructureNotifyMask;
        const FOCUS_CHANGE_MASK = xlib::FocusChangeMask;
        const PROPERTY_CHANGE_MASK = xlib::PropertyChangeMask;
        const COLORMAP_CHANGE_MASK = xlib::ColormapChangeMask;
        const OWNER_GRAB_BUTTON_MASK = xlib::OwnerGrabButtonMask;
    }
}

/// Non-generic trait for event wrappers.
pub trait DerivesAnEvent: Sized + Clone {
    /// Convert this item to an AnyEvent
    fn as_anyevent(self) -> AnyEvent;
    /// Get the type of this event.
    fn kind(&self) -> EventType {
        self.clone().as_anyevent().kind()
    }
    /// Get the window ID representing this event.
    fn window(&self) -> xlib::Window {
        self.clone().as_anyevent().window()
    }
    /// Is the event sent from the SendEvent function?
    fn from_send_event(&self) -> bool {
        self.clone().as_anyevent().from_send_event()
    }
}

/// Trait for event wrappers.
pub trait DerivesEvent<EvStruct>: DerivesAnEvent {
    /// Derive this item from the native struct.
    fn from_evstruct(xev: EvStruct) -> Result<Self, FlutterbugError>
    where
        Self: Sized;
}

/// The default XEvent
#[derive(Debug, Clone)]
pub struct AnyEvent {
    kind: EventType,
    window: xlib::Window,
    from_send_event: bool,
}

impl AnyEvent {
    /// Fill out with raw details
    #[inline]
    pub(crate) fn from_raw(kind: EventType, window: xlib::Window, fse: bool) -> Self {
        Self {
            kind,
            window,
            from_send_event: fse,
        }
    }
}

impl DerivesAnEvent for AnyEvent {
    #[inline]
    fn as_anyevent(self) -> Self {
        self
    }
    #[inline]
    fn kind(&self) -> EventType {
        self.kind
    }
    #[inline]
    fn window(&self) -> xlib::Window {
        self.window
    }
    #[inline]
    fn from_send_event(&self) -> bool {
        self.from_send_event
    }
}

// impl AnyEvent for all events
macro_rules! anyev_impl {
    ($xev: ty [ $winname: ident ]) => {
        impl DerivesEvent<$xev> for AnyEvent {
            #[inline]
            fn from_evstruct(xev: $xev) -> Result<Self, FlutterbugError> {
                Ok(Self {
                    kind: EventType::from_int(xev.type_)
                        .ok_or_else(|| FlutterbugError::InvalidEventType)?,
                    window: xev.$winname,
                    from_send_event: if xev.send_event != 0 { true } else { false },
                })
            }
        }
    };
    ($xev:ty) => {
        anyev_impl! {$xev[window]}
    };
}

anyev_impl! {xlib::XAnyEvent}
anyev_impl! {xlib::XKeyEvent}
anyev_impl! {xlib::XButtonEvent}
anyev_impl! {xlib::XMotionEvent}
anyev_impl! {xlib::XCrossingEvent}
anyev_impl! {xlib::XFocusChangeEvent}
anyev_impl! {xlib::XExposeEvent}
anyev_impl! {xlib::XGraphicsExposeEvent[drawable]}
anyev_impl! {xlib::XNoExposeEvent[drawable]}
anyev_impl! {xlib::XVisibilityEvent}
anyev_impl! {xlib::XCreateWindowEvent[parent]}
anyev_impl! {xlib::XDestroyWindowEvent}
anyev_impl! {xlib::XUnmapEvent}
anyev_impl! {xlib::XMapEvent}
anyev_impl! {xlib::XMapRequestEvent}
anyev_impl! {xlib::XReparentEvent}
anyev_impl! {xlib::XConfigureEvent}
anyev_impl! {xlib::XGravityEvent}
anyev_impl! {xlib::XResizeRequestEvent}
anyev_impl! {xlib::XConfigureRequestEvent}
anyev_impl! {xlib::XCirculateEvent}
anyev_impl! {xlib::XCirculateRequestEvent}
anyev_impl! {xlib::XPropertyEvent}
anyev_impl! {xlib::XSelectionClearEvent}
anyev_impl! {xlib::XSelectionRequestEvent[owner]}
anyev_impl! {xlib::XSelectionEvent[requestor]}
anyev_impl! {xlib::XColormapEvent}
anyev_impl! {xlib::XClientMessageEvent}
anyev_impl! {xlib::XMappingEvent[event]}
//anyev_impl!{xlib::XErrorEvent}
//anyev_impl!{xlib::XKeymapEvent}

// macro to create a new event type
macro_rules! event_type {
    ($(#[$attr: meta])* $vis: vis struct $sname: ident : $bname: ty [ $winname: ident ] {
        $($fname: ident : $ftname: ty = $sfname: ident),*
        $(,)?
    }) => {
        #[derive(Debug, Clone)]
        $(#[$attr])*
        $vis struct $sname {
            kind: EventType,
            window: xlib::Window,
            from_send_event: bool,
            $($fname: $ftname),*
        }

        impl DerivesAnEvent for $sname {
            #[inline]
            fn as_anyevent(self) -> AnyEvent {
                let Self { kind, window, from_send_event, .. } = self;
                AnyEvent::from_raw(kind, window, from_send_event)
            }

            #[inline]
            fn kind(&self) -> EventType { self.kind }
            #[inline]
            fn window(&self) -> xlib::Window { self.window }
            #[inline]
            fn from_send_event(&self) -> bool { self.from_send_event }
        }

        impl DerivesEvent<$bname> for $sname {
            #[inline]
            fn from_evstruct(ev: $bname) -> Result<Self, FlutterbugError> {
                Ok(Self {
                    kind: EventType::from_int(ev.type_).ok_or_else(|| FlutterbugError::InvalidEventType)?,
                    window: ev.$winname,
                    from_send_event: if ev.send_event != 0 { true } else { false },
                    $($fname: ev.$sfname as $ftname),*
                })
            }
        }

        impl $sname {
            $(#[inline] pub fn $fname(&self) -> $ftname { self.$fname })*
        }
    };
    ($(#[$attr: meta])* $vis: vis struct $sname: ident : $bname: ty {
        $($fname: ident : $ftname: ty = $sfname: ident),*
        $(,)?
    }) => {
        event_type! {
            $(#[$attr])*
            $vis struct $sname : $bname [ window ] {
                $($fname: $ftname = $sfname),*
            }
        }
    };
}

event_type! {
    pub struct KeyEvent : xlib::XKeyEvent {
        root: xlib::Window = root,
        subwindow: xlib::Window = subwindow,
        time: xlib::Time = time,
        x: u32 = x,
        y: u32 = y,
        x_root: u32 = x_root,
        y_root: u32 = y_root,
        state: c_uint = state,
        keycode: c_uint = keycode,
    }
}

event_type! {
    pub struct ButtonEvent : xlib::XButtonEvent {
        root: xlib::Window = root,
        subwindow: xlib::Window = subwindow,
        time: xlib::Time = time,
        x: u32 = x,
        y: u32 = y,
        x_root: u32 = x_root,
        y_root: u32 = y_root,
        state: c_uint = state,
        button: c_uint = button,
    }
}

event_type! {
    pub struct MotionEvent : xlib::XMotionEvent {
        root: xlib::Window = root,
        subwindow: xlib::Window = subwindow,
        time: xlib::Time = time,
        x: u32 = x,
        y: u32 = y,
        x_root: u32 = x_root,
        y_root: u32 = y_root,
        state: c_uint = state,
        is_hint: c_char = is_hint,
    }
}

event_type! {
    pub struct CrossingEvent : xlib::XCrossingEvent {
        root: xlib::Window = root,
        subwindow: xlib::Window = subwindow,
        time: xlib::Time = time,
        x: u32 = x,
        y: u32 = y,
        x_root: u32 = x_root,
        y_root: u32 = y_root,
        state: c_uint = state,
        mode: c_int = mode,
        detail: c_int = detail,
    }
}

event_type! {
    pub struct FocusChangeEvent : xlib::XFocusChangeEvent {
        mode: c_int = mode,
        detail: c_int = detail,
    }
}

event_type! {
    pub struct ExposeEvent : xlib::XExposeEvent {
        x: u32 = x,
        y: u32 = y,
        width: u32 = width,
        height: u32 = height,
        count: i32 = count,
    }
}

event_type! {
   pub struct NoExposeEvent : xlib::XNoExposeEvent[drawable] {
       major_code: c_int = major_code,
       minor_code: c_int = minor_code,
   }
}

event_type! {
    pub struct GraphicsExposeEvent : xlib::XGraphicsExposeEvent[drawable] {
        x: u32 = x,
        y: u32 = y,
        width: u32 = width,
        height: u32 = height,
        count: i32 = count,
        major_code: c_int = major_code,
        minor_code: c_int = minor_code,
    }
}

event_type! {
    pub struct ConfigureEvent : xlib::XConfigureEvent[event] {
        child: xlib::Window = window,
        x: u32 = x,
        y: u32 = y,
        width: u32 = width,
        height: u32 = height,
        border_width: u32 = border_width,
        above: xlib::Window = above,
    }
}

event_type! {
    pub struct VisibilityEvent : xlib::XVisibilityEvent {
        state: c_int = state,
    }
}

event_type! {
    pub struct CreateWindowEvent : xlib::XCreateWindowEvent[parent] {
        child: xlib::Window = window,
        x: u32 = x,
        y: u32 = y,
        width: u32 = width,
        height: u32 = height,
        border_width: u32 = border_width,
    }
}

event_type! {
    pub struct DestroyWindowEvent : xlib::XDestroyWindowEvent[event] {
        child: xlib::Window = window,
    }
}

event_type! {
    pub struct UnmapEvent : xlib::XUnmapEvent[event] {
        child: xlib::Window = window,
    }
}

event_type! {
    pub struct MapEvent : xlib::XMapEvent[event] {
        child: xlib::Window = window,
    }
}

event_type! {
    pub struct MapRequestEvent : xlib::XMapRequestEvent {
        child: xlib::Window = window,
    }
}

event_type! {
    pub struct ReparentEvent : xlib::XReparentEvent[event] {
        child: xlib::Window = window,
        parent: xlib::Window = parent,
        x: u32 = x,
        y: u32 = y,
    }
}

event_type! {
    pub struct GravityEvent : xlib::XGravityEvent[event] {
        child: xlib::Window = window,
        x: u32 = x,
        y: u32 = y,
    }
}

event_type! {
    pub struct ResizeRequestEvent : xlib::XResizeRequestEvent {
        width: u32 = width,
        height: u32 = height,
    }
}

event_type! {
    pub struct ConfigureRequestEvent : xlib::XConfigureRequestEvent {
        parent: xlib::Window = parent,
        x: u32 = x,
        y: u32 = y,
        width: u32 = width,
        height: u32 = height,
        border_width: u32 = border_width,
        above: xlib::Window = above,
        detail: c_int = detail,
        value_mask: c_uint = value_mask,
    }
}

event_type! {
    pub struct CirculateEvent : xlib::XCirculateEvent {
        event: xlib::Window = event,
        place: c_int = place,
    }
}

event_type! {
    pub struct CirculateRequestEvent : xlib::XCirculateRequestEvent {
        parent: xlib::Window = parent,
        place: c_int = place,
    }
}

event_type! {
    pub struct PropertyEvent : xlib::XPropertyEvent {
        atom: xlib::Atom = atom,
        time: xlib::Time = time,
        state: c_int = state,
    }
}

event_type! {
    pub struct SelectionClearEvent : xlib::XSelectionClearEvent {
        selection: xlib::Atom = selection,
        time: xlib::Time = time,
    }
}

event_type! {
    pub struct SelectionRequestEvent : xlib::XSelectionRequestEvent[owner] {
        requestor: xlib::Window = requestor,
        selection: xlib::Atom = selection,
        target: xlib::Atom = target,
        property: xlib::Atom = property,
        time: xlib::Time = time,
    }
}

event_type! {
    pub struct SelectionEvent : xlib::XSelectionEvent[requestor] {
        selection: xlib::Atom = selection,
        target: xlib::Atom = target,
        property: xlib::Atom = property,
        time: xlib::Time = time,
    }
}

event_type! {
    pub struct ColormapEvent : xlib::XColormapEvent {
        colormap: xlib::Colormap = colormap,
        state: c_int = state,
    }
}

event_type! {
    pub struct ClientMessageEvent : xlib::XClientMessageEvent {
        message_type: xlib::Atom = message_type,
        format: c_int = format,
        data: xlib::ClientMessageData = data,
    }
}

event_type! {
    pub struct MappingEvent : xlib::XMappingEvent[event] {
        request: c_int = request,
        first_keycode: c_int = first_keycode,
        count: i32 = count,
    }
}

event_type! {
    pub struct KeymapEvent : xlib::XKeymapEvent {
        keys: [c_char; 32] = key_vector
    }
}

/// An X11 event that can be received from the event loop.
#[derive(Debug, Clone)]
pub enum Event {
    Any(AnyEvent),
    Key(KeyEvent),
    Button(ButtonEvent),
    Motion(MotionEvent),
    Crossing(CrossingEvent),
    FocusChange(FocusChangeEvent),
    Expose(ExposeEvent),
    GraphicsExpose(GraphicsExposeEvent),
    NoExpose(NoExposeEvent),
    Visibility(VisibilityEvent),
    CreateWindow(CreateWindowEvent),
    DestroyWindow(DestroyWindowEvent),
    Unmap(UnmapEvent),
    Map(MapEvent),
    MapRequest(MapRequestEvent),
    Reparent(ReparentEvent),
    Configure(ConfigureEvent),
    Gravity(GravityEvent),
    ResizeRequest(ResizeRequestEvent),
    ConfigureRequest(ConfigureRequestEvent),
    Circulate(CirculateEvent),
    CirculateRequest(CirculateRequestEvent),
    Property(PropertyEvent),
    SelectionClear(SelectionClearEvent),
    SelectionRequest(SelectionRequestEvent),
    Selection(SelectionEvent),
    Colormap(ColormapEvent),
    ClientMessage(ClientMessageEvent),
    Mapping(MappingEvent),
    Keymap(KeymapEvent),
}

impl DerivesAnEvent for Event {
    fn as_anyevent(self) -> AnyEvent {
        match self {
            Event::Any(a) => a,
            Event::Key(k) => k.as_anyevent(),
            Event::Button(b) => b.as_anyevent(),
            Event::Motion(m) => m.as_anyevent(),
            Event::Crossing(c) => c.as_anyevent(),
            Event::FocusChange(fc) => fc.as_anyevent(),
            Event::Expose(e) => e.as_anyevent(),
            Event::GraphicsExpose(ge) => ge.as_anyevent(),
            Event::NoExpose(ne) => ne.as_anyevent(),
            Event::Visibility(v) => v.as_anyevent(),
            Event::CreateWindow(cw) => cw.as_anyevent(),
            Event::DestroyWindow(dw) => dw.as_anyevent(),
            Event::Unmap(u) => u.as_anyevent(),
            Event::Map(m) => m.as_anyevent(),
            Event::MapRequest(mr) => mr.as_anyevent(),
            Event::Reparent(r) => r.as_anyevent(),
            Event::Configure(c) => c.as_anyevent(),
            Event::Gravity(g) => g.as_anyevent(),
            Event::ResizeRequest(rr) => rr.as_anyevent(),
            Event::ConfigureRequest(cr) => cr.as_anyevent(),
            Event::Circulate(c) => c.as_anyevent(),
            Event::CirculateRequest(cr) => cr.as_anyevent(),
            Event::Property(p) => p.as_anyevent(),
            Event::SelectionClear(sc) => sc.as_anyevent(),
            Event::SelectionRequest(sr) => sr.as_anyevent(),
            Event::Selection(s) => s.as_anyevent(),
            Event::Colormap(cm) => cm.as_anyevent(),
            Event::ClientMessage(cm) => cm.as_anyevent(),
            Event::Mapping(m) => m.as_anyevent(),
            Event::Keymap(k) => k.as_anyevent(),
        }
    }
}

impl DerivesEvent<xlib::XEvent> for Event {
    fn from_evstruct(x: xlib::XEvent) -> Result<Self, FlutterbugError> {
        let kind = unsafe { x.type_ };
        let kind = EventType::from_int(kind).ok_or_else(|| FlutterbugError::InvalidEventType)?;

        macro_rules! evt {
            ($bname: ident, $sname: ty, $evfield: ident) => {
                Ok(Event::$bname(<$sname>::from_evstruct(unsafe {
                    x.$evfield
                })?))
            };
        }

        match kind {
            EventType::KeyPress | EventType::KeyRelease => evt!(Key, KeyEvent, key),
            EventType::ButtonPress | EventType::ButtonRelease => evt!(Button, ButtonEvent, button),
            EventType::MotionNotify => evt!(Motion, MotionEvent, motion),
            EventType::FocusIn | EventType::FocusOut => {
                evt!(FocusChange, FocusChangeEvent, focus_change)
            }
            EventType::EnterNotify | EventType::LeaveNotify => {
                evt!(Crossing, CrossingEvent, crossing)
            }
            EventType::KeymapNotify => evt!(Keymap, KeymapEvent, keymap),
            EventType::Expose => evt!(Expose, ExposeEvent, expose),
            EventType::GraphicsExpose => evt!(GraphicsExpose, GraphicsExposeEvent, graphics_expose),
            EventType::NoExpose => evt!(NoExpose, NoExposeEvent, no_expose),
            EventType::CirculateRequest => {
                evt!(CirculateRequest, CirculateRequestEvent, circulate_request)
            }
            EventType::ConfigureRequest => {
                evt!(ConfigureRequest, ConfigureRequestEvent, configure_request)
            }
            EventType::MapRequest => evt!(MapRequest, MapRequestEvent, map_request),
            EventType::ResizeRequest => evt!(ResizeRequest, ResizeRequestEvent, resize_request),
            EventType::CirculateNotify => evt!(Circulate, CirculateEvent, circulate),
            EventType::ConfigureNotify => evt!(Configure, ConfigureEvent, configure),
            EventType::CreateNotify => evt!(CreateWindow, CreateWindowEvent, create_window),
            EventType::DestroyNotify => evt!(DestroyWindow, DestroyWindowEvent, destroy_window),
            EventType::GravityNotify => evt!(Gravity, GravityEvent, gravity),
            EventType::MapNotify => evt!(Map, MapEvent, map),
            EventType::MappingNotify => evt!(Mapping, MappingEvent, mapping),
            EventType::ReparentNotify => evt!(Reparent, ReparentEvent, reparent),
            EventType::UnmapNotify => evt!(Unmap, UnmapEvent, unmap),
            EventType::VisibilityNotify => evt!(Visibility, VisibilityEvent, visibility),
            EventType::ColormapEvent => evt!(Colormap, ColormapEvent, colormap),
            EventType::ClientMessage => evt!(ClientMessage, ClientMessageEvent, client_message),
            EventType::PropertyNotify => evt!(Property, PropertyEvent, property),
            EventType::SelectionClear => evt!(SelectionClear, SelectionClearEvent, selection_clear),
            EventType::SelectionRequest => {
                evt!(SelectionRequest, SelectionRequestEvent, selection_request)
            }
            EventType::SelectionNotify => evt!(Selection, SelectionEvent, selection),
            _ => evt!(Any, AnyEvent, any),
        }
    }
}
