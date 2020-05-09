/* -----------------------------------------------------------------------------------
 * src/widget/mod.rs
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

pub mod internal;
use internal::*;
mod reference;
pub use reference::*;

use crate::object::GuiObject;
use nalgebra::geometry::Point4;
use owning_ref::{RefMutRefMut, RefRef};
use std::{cell::RefCell, fmt, sync::Arc};

/// A GUI widget.
#[derive(Debug)]
pub struct Widget<Inner: GuiObject + 'static> {
    internal: Arc<RefCell<WidgetInternal<Inner>>>,
    generic_ref: Arc<RefCell<dyn GenericWidgetInternal>>
}

impl<Inner: GuiObject + 'static> Widget<Inner> {
    /// Create a new Widget from the internal Arc.
    #[inline]
    pub(crate) fn from_internal(internal: Arc<RefCell<WidgetInternal<Inner>>>) -> Self {
        Self { generic_ref: internal.clone(), internal }
    }

    /// Get the internal Arc of the Widget.
    #[inline]
    pub(crate) fn internal(&self) -> &Arc<RefCell<WidgetInternal<Inner>>> {
        &self.internal
    }
}

/// Trait that applies to all GUI widgets.
pub trait GenericWidget: fmt::Debug {
    /// The ID of this widget that uniquely identifies it.
    fn id(&self) -> Result<u64, crate::Error>;
    /// Convert this item to a generic reference.
    fn generic_reference(&self) -> GenericWidgetReference;

    /// A generic reference to the internal Arc container.
    fn internal_generic(&self) -> Result<&Arc<RefCell<dyn GenericWidgetInternal>>, crate::Error>;
    /// A generic reference to the inner peer object.
    fn inner_generic(
        &self,
    ) -> Result<RefRef<'_, dyn GenericWidgetInternal, dyn GuiObject>, crate::Error>;
    /// A mutable generic reference to the inner peer object.
    fn inner_generic_mut(
        &self,
    ) -> Result<RefMutRefMut<'_, dyn GenericWidgetInternal, dyn GuiObject>, crate::Error>;

    /// The bounds (x/y/width/height) of this widget.
    fn bounds(&self) -> Result<Point4<u32>, crate::Error>;
    /// Set the bounds (x/y/width/height) of this widget.
    fn set_bounds(&self, bounds: Point4<u32>) -> Result<(), crate::Error>;

    /// The parent widget for this object.
    fn parent(
        &self,
    ) -> Result<RefRef<'_, dyn GenericWidgetInternal, Option<GenericWidgetReference>>, crate::Error>;
    /// Set the parent widget for this object.
    ///
    /// Note: This will also add this widget as a child for the other item.
    fn set_parent(&self, parent: &dyn GenericWidget) -> Result<(), crate::Error>;

    /// The list of children for this object.
    fn children(
        &self,
    ) -> Result<RefRef<'_, dyn GenericWidgetInternal, [GenericWidgetReference]>, crate::Error>;
    /// Add a child to this widget.
    fn add_child(&self, child: &dyn GenericWidget) -> Result<(), crate::Error>;
    /// Remove a child from this widget.
    fn remove_child(&self, child: &dyn GenericWidget) -> Result<(), crate::Error>;
}

/// helper function for setting parent
pub(crate) fn set_parent_internal(
    parent: GenericWidgetReference,
    child: GenericWidgetReference,
) -> Result<(), crate::Error> {
    // remove from current parent's children list
    let imm_borrow = child.internal_generic()?.try_borrow()?;
    if let Some(current_parent) = imm_borrow.parent() {
        current_parent
            .internal_generic()?
            .try_borrow_mut()?
            .remove_child(imm_borrow.id())?;
    }

    child
        .internal_generic()?
        .try_borrow_mut()?
        .set_parent(Some(parent.clone()))?;
    parent
        .internal_generic()?
        .try_borrow_mut()?
        .add_child(&child)
}

#[doc(hidden)]
#[macro_export]
macro_rules! forward_to_i_generic {
    () => {
        #[inline]
        fn id(&self) -> Result<u64, crate::Error> {
            Ok(self.internal_generic()?.try_borrow()?.id())
        }
        #[inline]
        fn inner_generic(
            &self,
        ) -> Result<RefRef<'_, dyn GenericWidgetInternal, dyn GuiObject>, crate::Error> {
            Ok(RefRef::new(self.internal_generic()?.try_borrow()?).map(|r| r.inner_generic()))
        }
        #[inline]
        fn inner_generic_mut(
            &self,
        ) -> Result<RefMutRefMut<'_, dyn GenericWidgetInternal, dyn GuiObject>, crate::Error> {
            Ok(
                RefMutRefMut::new(self.internal_generic()?.try_borrow_mut()?)
                    .map_mut(|r| r.inner_generic_mut()),
            )
        }

        #[inline]
        fn bounds(&self) -> Result<Point4<u32>, crate::Error> {
            Ok(self.internal_generic()?.try_borrow()?.bounds())
        }
        #[inline]
        fn set_bounds(&self, bounds: Point4<u32>) -> Result<(), crate::Error> {
            self.internal_generic()?
                .try_borrow_mut()?
                .set_bounds(bounds)
        }

        #[inline]
        fn parent(
            &self,
        ) -> Result<
            RefRef<'_, dyn GenericWidgetInternal, Option<GenericWidgetReference>>,
            crate::Error,
        > {
            Ok(RefRef::new(self.internal_generic()?.try_borrow()?).map(|r| r.parent()))
        }
        #[inline]
        fn set_parent(&self, parent: &dyn GenericWidget) -> Result<(), crate::Error> {
            set_parent_internal(parent.generic_reference(), self.generic_reference())
        }

        #[inline]
        fn children(
            &self,
        ) -> Result<RefRef<'_, dyn GenericWidgetInternal, [GenericWidgetReference]>, crate::Error> {
            Ok(RefRef::new(self.internal_generic()?.try_borrow()?).map(|r| r.children()))
        }
        #[inline]
        fn add_child(&self, child: &dyn GenericWidget) -> Result<(), crate::Error> {
            child.set_parent(self)
        }
        #[inline]
        fn remove_child(&self, child: &dyn GenericWidget) -> Result<(), crate::Error> {
            self.internal_generic()?
                .try_borrow_mut()?
                .remove_child(child.id()?)
        }
    };
}

impl<Inner: GuiObject + 'static> GenericWidget for Widget<Inner> {
    #[inline]
    fn internal_generic(&self) -> Result<&Arc<RefCell<dyn GenericWidgetInternal>>, crate::Error> {
        Ok(&self.generic_ref)
    }

    #[inline]
    fn generic_reference(&self) -> GenericWidgetReference {
        let generic: Arc<RefCell<dyn GenericWidgetInternal>> = self.internal.clone();
        GenericWidgetReference::from_reference(generic)
    }

    forward_to_i_generic! {}
}
