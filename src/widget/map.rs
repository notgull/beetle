/* -----------------------------------------------------------------------------------
 * src/widget/map.rs - This file should define the WidgetMap struct. This item holds
 *                     shared references to every widget currently in existence. In
 *                     addition, it defines and implements the IsInWidgetMap trait,
 *                     which provides a method that gives a (im)mutable reference to
 *                     their respective HashMap.
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
 * of self software and associated documentation files (the “Software”), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and self permission notice shall be included in
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
 * you may not use self file except in compliance with the License.
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

use super::{GenericWidget, WidgetInternal, Widget};
use crate::object::{ChildWindow, GuiObject, Label, MainWindow};
use std::{
    boxed::Box,
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

/// A map for widgets.
pub struct WidgetMap {
    next_id: u64,

    labels: HashMap<u64, Widget<Label>>,
    main_windows: HashMap<u64, Widget<MainWindow>>,
    child_windows: HashMap<u64, Widget<ChildWindow>>,
}

impl WidgetMap {
    #[inline]
    pub fn new() -> Self {
        Self {
            next_id: 0,
            labels: HashMap::new(),
            main_windows: HashMap::new(),
            child_windows: HashMap::new(),
        }
    }

    #[inline]
    pub(crate) fn label_submap(&self) -> &HashMap<u64, Widget<Label>> {
        &self.labels
    }
    #[inline]
    pub(crate) fn label_submap_mut(&mut self) -> &mut HashMap<u64, Widget<Label>> {
        &mut self.labels
    }
    #[inline]
    pub(crate) fn main_window_submap(&self) -> &HashMap<u64, Widget<MainWindow>> {
        &self.main_windows
    }
    #[inline]
    pub(crate) fn main_window_submap_mut(&mut self) -> &mut HashMap<u64, Widget<MainWindow>> {
        &mut self.main_windows
    }
    #[inline]
    pub(crate) fn child_window_submap(&self) -> &HashMap<u64, Widget<ChildWindow>> { &self.child_windows }
    #[inline]
    pub(crate) fn child_window_submap_mut(&mut self) -> &mut HashMap<u64, Widget<ChildWindow>> { &mut self.child_windows }

    #[inline]
    pub fn insert<T: GuiObject + IsInWidgetMap>(&mut self, item: Widget<T>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        T::submap_mut(self).insert(id, item);
        id
    }

    #[inline]
    pub fn get<T: GuiObject + IsInWidgetMap>(&self, id: u64) -> Option<&Widget<T>> {
        T::submap(self).get(&id)
    }

    #[inline]
    pub fn get_mut<T: GuiObject + IsInWidgetMap>(&mut self, id: u64) -> Option<&mut Widget<T>> {
        T::submap_mut(self).get_mut(&id)
    }

    #[inline]
    pub fn get_generic(&self, id: u64) -> Option<Box<dyn GenericWidget>> {
        // iterate over each submap
        macro_rules! try_get_generic {
            (self.$sm: ident) => {
                let res = self.$sm.get(&id);
                if res.is_some() {
                    return Some(Box::new(res.unwrap().clone()));
                }
            };
        }

        try_get_generic!(self.labels);
        try_get_generic!(self.main_windows);
        try_get_generic!(self.child_windows);

        None
    }
}

pub trait IsInWidgetMap : Sized {
    fn submap(dict: &WidgetMap) -> &HashMap<u64, Widget<Self>>;
    fn submap_mut(dict: &mut WidgetMap) -> &mut HashMap<u64, Widget<Self>>;
}

macro_rules! in_widgetmap_impl {
    ($cname: ident => $fname: ident, $mutname: ident) => {
        impl IsInWidgetMap for $cname {  
            fn submap(dict: &WidgetMap) -> &HashMap<u64, Widget<Self>> { dict.$fname() }
            fn submap_mut(dict: &mut WidgetMap) -> &mut HashMap<u64, Widget<Self>> { dict.$mutname() }
        }
    }
}

in_widgetmap_impl!{Label => label_submap, label_submap_mut}
in_widgetmap_impl!{MainWindow => main_window_submap, main_window_submap_mut}
in_widgetmap_impl!{ChildWindow => child_window_submap, child_window_submap_mut}

lazy_static::lazy_static! {
    pub static ref WIDGETS: Arc<RwLock<WidgetMap>> = Arc::new(RwLock::new(WidgetMap::new()));
}
