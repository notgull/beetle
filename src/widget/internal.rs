/* -----------------------------------------------------------------------------------
 * src/widget/internal.rs - This file should define the WidgetInternal structure. It
 *                          will hold references to the actual peer object, as well
 *                          as other variables such as bounds, text, parent, etc.
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

use super::{GenericWidget, GenericWidgetReference};
use crate::object::GuiObject;
use nalgebra::geometry::Point4;
use std::fmt;

/// A wrapper around the peer object.
#[derive(Debug)]
pub struct WidgetInternal<Inner: GuiObject + 'static> {
    // a unique ID assigned to every widget
    id: u64,
    // wrapping peer object
    inner: Inner,
    // xywh of peer object
    bounds: Point4<u32>,
    // text/title of peer object
    text: String,
    // object's parent, or None if it doesn't have one
    parent: Option<GenericWidgetReference>,
    // object's children
    children: Vec<GenericWidgetReference>,
}

impl<Inner: GuiObject + 'static> WidgetInternal<Inner> {
    #[inline]
    pub(crate) fn inner(&self) -> &Inner {
        &self.inner
    }

    #[inline]
    pub(crate) fn inner_mut(&mut self) -> &mut Inner {
        &mut self.inner
    }
}

/// A trait that is applied to all WidgetInternal objects.
pub trait GenericWidgetInternal: fmt::Debug {
    /// Get this widget's ID.
    fn id(&self) -> u64;
    /// Get a generic reference to the internal peer object.
    fn inner_generic(&self) -> &(dyn GuiObject + 'static);
    /// Get a mutable generic reference to the internal peer object.
    fn inner_generic_mut(&mut self) -> &mut (dyn GuiObject + 'static);
    /// Get the bounds of this object.
    fn bounds(&self) -> Point4<u32>;
    /// Set the bounds of this object.
    fn set_bounds(&mut self, bounds: Point4<u32>) -> Result<(), crate::Error>;
    /// Get the parent of this object.
    fn parent(&self) -> &Option<GenericWidgetReference>;
    /// Set the parent of this object.
    fn set_parent(&mut self, parent: Option<GenericWidgetReference>) -> Result<(), crate::Error>;
    /// Get the children of this object.
    fn children(&self) -> &[GenericWidgetReference];
    /// Add a child to this object.
    fn add_child(&mut self, child: &GenericWidgetReference) -> Result<(), crate::Error>;
    /// Remove a child from this object.
    fn remove_child(&mut self, child_id: u64) -> Result<(), crate::Error>;
}

// implement GenericWidgetInternal for all WidgetInternal
impl<Inner: GuiObject + 'static> GenericWidgetInternal for WidgetInternal<Inner> {
    // most of these functions can be inlined
    #[inline]
    fn id(&self) -> u64 {
        self.id
    }
    #[inline]
    fn inner_generic(&self) -> &(dyn GuiObject + 'static) {
        &self.inner
    }
    #[inline]
    fn inner_generic_mut(&mut self) -> &mut (dyn GuiObject + 'static) {
        &mut self.inner
    }
    #[inline]
    fn bounds(&self) -> Point4<u32> {
        self.bounds
    }
    #[inline]
    fn set_bounds(&mut self, bounds: Point4<u32>) -> Result<(), crate::Error> {
        self.bounds = bounds;
        self.inner.set_bounds(bounds)
    }
    #[inline]
    fn parent(&self) -> &Option<GenericWidgetReference> {
        &self.parent
    }
    #[inline]
    fn children(&self) -> &[GenericWidgetReference] {
        &self.children
    }

    // set this object's parent, as well as the peer object's parent
    #[inline]
    fn set_parent(&mut self, parent: Option<GenericWidgetReference>) -> Result<(), crate::Error> {
        match parent {
            Some(p) => {
                use std::ops::Deref;
                self.inner.set_parent(p.inner_generic()?.deref())?;
                self.parent = Some(p);
                Ok(())
            }
            None => Err(crate::Error::StaticMsg(
                "Unfortunately, we do not yet support removing a widget's parents.",
            )),
        }
    }

    // add a child to this object
    #[inline]
    fn add_child(&mut self, child: &GenericWidgetReference) -> Result<(), crate::Error> {
        self.children.push(child.clone());
        Ok(())
    }

    #[inline]
    fn remove_child(&mut self, child_id: u64) -> Result<(), crate::Error> {
        self.children.retain(|c| c.id().unwrap() != child_id);
        Ok(())
    }
}
