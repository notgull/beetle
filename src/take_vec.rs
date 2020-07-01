/* -----------------------------------------------------------------------------------
 * src/take_vec.rs - A container that holds an item and the number of times it can be
 *                   "taken" out of the container.
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

// this item is only used for the Win32 version of beetle
#![cfg(windows)]
#![allow(dead_code)]

use std::{fmt, mem};

/// A container, where every item is identical, and a certain number of items can be
/// "taken" from it.
pub struct TakeVec<T: Clone> {
    value: Option<T>,
    capacity: usize,
}

impl<T: fmt::Debug + Clone> fmt::Debug for TakeVec<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TakeVec")
            .field("value", &self.value)
            .field("capacity", &self.capacity)
            .finish()
    }
}

impl<T: Clone> TakeVec<T> {
    /// Create a new, empty TakeVec.
    #[inline]
    pub fn new() -> Self {
        TakeVec {
            value: None,
            capacity: 0,
        }
    }

    /// Put an item into this TakeVec. It will return the previous item stored in this TakeVec.
    #[inline]
    pub fn push(&mut self, item: T) -> Option<T> {
        let mut value = Some(item);
        mem::swap(&mut self.value, &mut value);
        self.increment();
        value
    }

    /// Increment the counter on this TakeVec without replacing the value.
    #[inline]
    pub fn increment(&mut self) {
        self.capacity += 1;
    }

    /// Take an item out of this TakeVec. If the capacity after the operation is greater than 0,
    /// it will return a clone of the item instead.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        match self.capacity {
            0 => None,
            1 => {
                self.capacity = 0;
                self.value.take()
            }
            _ => {
                self.capacity -= 1;
                self.value.clone()
            }
        }
    }

    /// Get the current capacity of the TakeVec.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Tell if this container is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        debug_assert!(self.value.is_none() && self.capacity == 0);
        self.value.is_none()
    }

    /// If the container is empty, add a new value. Otherwise, increment
    /// the capacity by one.
    #[inline]
    pub fn store(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
            self.capacity = 1;
        } else {
            self.capacity += 1;
            // TODO: it might be prudent to make sure "value" isn't dropped here.
            //       since this is only used internally with euclid values it
            //       shouldn't be an issue
        }
    }
}

#[test]
fn test_take_vec() {
    use euclid::default::Rect;
    let rect1 = euclid::rect(1, 2, 3, 4);
    let rect2 = euclid::rect(5, 6, 7, 8);

    let mut tv = TakeVec::new();
    assert!(tv.is_empty());

    tv.push(rect1);
    assert!(!tv.is_empty());
    assert_eq!(tv.capacity(), 1);

    tv.push(rect2);
    assert_eq!(tv.capacity(), 2);

    tv.increment();
    assert_eq!(tv.capacity(), 3);

    tv.store(rect1);
    assert_eq!(tv.capacity(), 4);

    for i in 0..4 {
        let r = tv.take().unwrap();
        assert_eq!(r, rect2);
    }

    assert!(tv.is_empty());
    assert!(tv.take().is_none());
}
