/* -----------------------------------------------------------------------------------
 * src/object/x11/window.rs - The X11 window peer object.
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

use super::{BasicWidgetProperties, X11GuiFactory};
use crate::{
    object::{ChildWindowBase, ContainerBase, MainWindowBase, PeerObject, WindowBase},
    GenericWidget, GenericWidgetReference, Signal, SigDestroyWindow, SigDestroyApplication, 
};
use euclid::default::{Point2D, Rect, Size2D};
use flutterbug::{prelude::*, x11::xlib, Atom, Event, EventMask, Window};
use std::{
    fmt,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

/// A type of X11 window.
pub trait X11WindowType: fmt::Debug {
    /// The expected type of the parent.
    type ParentType;

    /// Is the window the main window?
    fn is_main() -> bool;
    /// Given a ParentType, get the window to pass through.
    fn get_parent(p: Self::ParentType) -> Option<GenericWidgetReference>;
}

/// The type for the main window.
#[derive(Debug)]
pub struct X11MainWindow;

impl X11WindowType for X11MainWindow {
    type ParentType = ();

    #[inline]
    fn is_main() -> bool {
        true
    }
    #[inline]
    fn get_parent(_p: ()) -> Option<GenericWidgetReference> {
        None
    }
}

/// The type for the child windows.
#[derive(Debug)]
pub struct X11ChildWindow;

impl X11WindowType for X11ChildWindow {
    type ParentType = GenericWidgetReference;

    #[inline]
    fn is_main() -> bool {
        false
    }
    #[inline]
    fn get_parent(p: GenericWidgetReference) -> Option<GenericWidgetReference> {
        Some(p)
    }
}

/// An X11 frame.
#[derive(Debug)]
pub struct X11Window<WindowType: X11WindowType> {
    props: BasicWidgetProperties,
    window: Window,
    wdw_atom: Atom,
    _phantom: PhantomData<WindowType>,
}

impl<WindowType: X11WindowType> X11Window<WindowType> {
    pub(crate) fn new(
        factory: &X11GuiFactory,
        _parent: WindowType::ParentType,
        bounds: Rect<u32>,
    ) -> Result<Self, crate::Error> {
        let origin = super::get_x_origin(bounds);
        let black = factory.display().default_black_pixel()?;
        let white = factory.display().default_white_pixel()?;
        let sz = bounds.size;
                
        let window = factory.display().create_simple_window(None, origin, sz, 1, black, white)?;

        // also get the delete window atom
        let wdw = factory.display().internal_atom("WM_DELETE_WINDOW", false)?;
        window.set_protocols(&mut [wdw])?;
 
        window.select_input(EventMask::EXPOSURE_MASK)?;

        Ok(Self {
            props: BasicWidgetProperties::new(bounds),
            window,
            wdw_atom: wdw,
            _phantom: PhantomData,
        })
    }
}

impl<WindowType: X11WindowType> PeerObject for X11Window<WindowType> {
    #[inline]
    fn set_bounds(&mut self, bounds: Rect<u32>) -> Result<(), crate::Error> {
        Ok(self
            .window
            .set_bounds(super::get_x_origin(bounds), bounds.size)?)
    }

    #[inline]
    fn set_parent(&mut self, parent: &dyn PeerObject) -> Result<(), crate::Error> {
        // TODO; flutterbug set_parent
        unimplemented!()
    }

    #[inline]
    fn internal_x11_window(&self) -> &Window {
        &self.window
    }

    #[inline]
    fn translate_x11_event(&mut self, xev: Event) -> Result<Vec<Arc<dyn Signal + 'static>>, crate::Error> {
        super::props_x11_event(&xev, &self.window, &mut self.props)?;

        // check to see if we need to close the window
        let mut signals: Vec<Arc<dyn Signal + 'static>> = vec![];

        match &xev {
            Event::ClientMessage(ref cm) => {
                if AsRef::<[Atom]>::as_ref(&cm.data())[0] == self.wdw_atom {
                    if WindowType::is_main() {
                        signals.push(Arc::new(SigDestroyApplication));
                    } else {
                        signals.push(Arc::new(SigDestroyWindow));
                    }
                }
            }
            _ => { /* do nothing */ }
        }

        signals.extend(super::default_x11_translate_event(xev)?);
        Ok(signals)
    }
}

impl<WindowType: X11WindowType> WindowBase for X11Window<WindowType> {
    #[inline]
    fn set_title(&mut self, title: String) -> Result<(), crate::Error> {
        Ok(self
            .window
            .set_standard_properties(Some(&title), Some(&title), None, false)?)
    }

    #[inline]
    fn display(&self) -> Result<(), crate::Error> {
        Ok(self.window.map(false)?)
    }
}

impl MainWindowBase for X11Window<X11MainWindow> {}
impl ChildWindowBase for X11Window<X11ChildWindow> {}
impl<WindowType: X11WindowType> ContainerBase for X11Window<WindowType> {}
