/* -----------------------------------------------------------------------------------
 * src/widget/reference.rs - Defines the GenericWidgetReference struct, which is a
 *                           Weak wrapper around a `dyn GenericWidgetInternal`. It
 *                           should forward its calls to the internal structure.
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

use super::{set_parent_internal, GenericWidget, GenericWidgetInternal};
use crate::{forward_to_i_generic, object::GuiObject};
use nalgebra::geometry::Point4;
use owning_ref::{RefMutRefMut, RefRef};
use std::{
    cell::RefCell,
    sync::Arc,
};

/// A generic reference to a widget.
#[derive(Debug)]
pub struct GenericWidgetReference {
    reference: Arc<RefCell<dyn GenericWidgetInternal>>,
}

impl Clone for GenericWidgetReference {
    fn clone(&self) -> Self {
        Self::from_reference(self.reference().clone())
    }
}

impl GenericWidgetReference {
    /// Create a GenericWidgetReference from the raw reference.
    #[inline]
    pub(crate) fn from_reference(reference: Arc<RefCell<dyn GenericWidgetInternal>>) -> Self {
        Self { reference }
    }

    /// Get the internal reference object.
    #[inline]
    pub(crate) fn reference(&self) -> &Arc<RefCell<dyn GenericWidgetInternal>> {
        &self.reference
    }
}

impl GenericWidget for GenericWidgetReference {
    #[inline]
    fn internal_generic(&self) -> Result<&Arc<RefCell<dyn GenericWidgetInternal>>, crate::Error> {
        Ok(self.reference())
    }

    #[inline]
    fn generic_reference(&self) -> GenericWidgetReference {
        self.clone()
    }

    forward_to_i_generic! {}
}