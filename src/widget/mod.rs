/* -----------------------------------------------------------------------------------
 * src/widget/mod.rs - This file should defined the Widget struct. The Widget struct
 *                     is the item that represents all widgets on an internal level.
 *                     Widget should be defined as a wrapper around a shared reference
 *                     to the actual Widget object, in order to simplify some of the
 *                     semantics.
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

pub mod map;
use map::*;
mod kind;
pub use kind::*;

use crate::{
    object::{GuiFactory, GuiObject},
    Color,
};
use nalgebra::geometry::Point4;
use std::{
    boxed::Box, 
    fmt,
    mem,
    ops::Deref,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Debug)]
pub struct WidgetInternal<Inner: IsInWidgetMap> {
    id: Option<u64>,
    inner: Inner,
    bounds: Point4<u32>,
    text: String,
    parent: Option<u64>,
    children: Vec<u64>,
}

impl<Inner: GuiObject + IsInWidgetMap> WidgetInternal<Inner> {
    #[inline]
    pub(crate) fn empty(inner: Inner) -> Self {
        Self {
            id: None,
            inner,
            bounds: Point4::new(0, 0, 0, 0),
            text: String::new(),
            parent: None,
            children: vec![],
        }
    }

    #[inline]
    pub(crate) fn create_rc(self) -> Result<Widget<Inner>, crate::Error> {
        let reference = Arc::new(RwLock::new(self));
        let w = Widget::from_internal(reference);
        let id = WIDGETS.try_write()?.insert(w.clone());
        w.internal().try_write()?.set_id(id);
        Ok(w)
    }

    #[inline]
    pub(crate) fn id(&self) -> u64 {
        self.id.expect("Widget has not yet been assigned its id")
    }

    #[inline]
    pub(crate) fn set_id(&mut self, id: u64) {
        self.id = Some(id)
    }

    #[inline]
    pub(crate) fn inner(&self) -> &Inner {
        &self.inner
    }

    #[inline]
    pub(crate) fn inner_mut(&mut self) -> &mut Inner {
        &mut self.inner
    }

    #[inline]
    pub(crate) fn bounds(&self) -> Point4<u32> {
        self.bounds
    }
    #[inline]
    pub(crate) fn set_bounds(&mut self, bounds: Point4<u32>) -> Result<(), crate::Error> {
        self.bounds = bounds;
        self.inner_mut().set_bounds(bounds)
    }

    #[inline]
    pub(crate) fn parent_direct(&self) -> Option<u64> {
        self.parent
    }
    #[inline]
    pub(crate) fn set_parent_direct(&mut self, parent: Option<u64>) {
        self.parent = parent
    }
    #[inline]
    pub(crate) fn children_direct(&self) -> &[u64] {
        &self.children
    }
    #[inline]
    pub(crate) fn add_child_direct(&mut self, child: u64) {
        self.children.push(child)
    }
    #[inline]
    pub(crate) fn remove_child_direct(&mut self, child: u64) {
        if let Some(i) = self.children.iter().position(|c| *c == child) {
            self.children.remove(i);
        }
    }
}

/// A trait that describes a widget of any type.
pub trait GenericWidget {
    fn bounds(&self) -> Point4<u32>;
    fn set_bounds(&mut self, bounds: Point4<u32>) -> Result<(), crate::Error>;

    #[doc(hidden)]
    fn remove_child_direct(&mut self, id: u64);
}

impl<Inner: GuiObject + IsInWidgetMap> GenericWidget for Widget<Inner> {
    #[inline]
    fn bounds(&self) -> Point4<u32> {
        self.internal.try_read().unwrap().bounds()
    }

    #[inline]
    fn set_bounds(&mut self, bounds: Point4<u32>) -> Result<(), crate::Error> {
        // TODO: update container stuff
        self.internal.try_write()?.set_bounds(bounds)
    }

    #[inline]
    fn remove_child_direct(&mut self, id: u64) { self.internal.try_write().unwrap().remove_child_direct(id) }
}

impl<Inner: GuiObject + IsInWidgetMap> Widget<Inner> {
    pub fn set_parent<T: GuiObject + IsInWidgetMap>(&mut self, parent: &mut Widget<T>) -> Result<(), crate::Error> {
        let imm_borrow = self.internal.try_read()?;
        if let Some(p) = imm_borrow.parent_direct() {
            WIDGETS
                .try_read()?
                .get_generic(p)
                .ok_or_else(|| crate::Error::WidgetMapMissingId(p))?
                .remove_child_direct(imm_borrow.id());
        }
        mem::drop(imm_borrow);

        {
            // scoped to drop mut lock
            self.internal
                .try_write()?
                .set_parent_direct(Some(parent.internal().try_read()?.id()));
            parent
                .internal()
                .try_write()?
                .add_child_direct(self.internal.try_read()?.id());
        }

        // TODO: rearrange layout manager
        self.internal.try_write()?
            .inner_mut().set_parent(parent.internal().try_read()?.inner())
    }

    #[inline]
    pub fn parent(&self) -> Option<Box<dyn GenericWidget>> {
        match self.internal.try_read().unwrap().parent_direct() {
            None => None,
            Some(p) => WIDGETS
                .try_read().unwrap()
                .get_generic(p)
        }
    }

    #[inline]
    fn children(&self) -> Vec<Box<dyn GenericWidget>> {
        self.internal
            .try_read().unwrap()
            .children_direct()
            .iter()
            .filter_map(|r| {
                WIDGETS
                    .try_read().unwrap()
                    .get_generic(*r)
            })
            .collect()
    } 

    #[inline]
    fn child(&self, index: usize) -> Option<Box<dyn GenericWidget>> {
        let imm_borrow = self.internal.try_read().unwrap();
        if index >= imm_borrow.children_direct().len() {
            None
        } else {
            WIDGETS
                .try_read()
                .unwrap()
                .get_generic(imm_borrow.children_direct()[index])
        }
    }

    #[inline]
    fn add_child<T: GuiObject + IsInWidgetMap>(&mut self, child: &mut Widget<T>) -> Result<(), crate::Error> {
        child.set_parent(self)
    }
}

#[derive(Debug)]
pub struct Widget<Inner: IsInWidgetMap> {
    internal: Arc<RwLock<WidgetInternal<Inner>>>,
}

impl<Inner: IsInWidgetMap> Widget<Inner> {
    /// Gets the shared reference containing the internal widget.
    #[inline]
    pub fn internal(&self) -> &Arc<RwLock<WidgetInternal<Inner>>> {
        &self.internal
    }

    /// Create a widget that just wraps an Rc around the internal object
    #[inline]
    pub(crate) fn from_internal(internal: Arc<RwLock<WidgetInternal<Inner>>>) -> Self {
        Self { internal }
    }
}

impl<Inner: IsInWidgetMap> Clone for Widget<Inner> {
    fn clone(&self) -> Self {
        Self::from_internal(self.internal.clone())
    }
}
