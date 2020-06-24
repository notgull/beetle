/* -----------------------------------------------------------------------------------
 * src/ro_mmg.rs - Read Only Mapped Mutex Guard. A simple wrapper around the
 *                 parking_lot MappedMutexGuard type that derefs to &T instead of
 *                 &mut T.
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

use parking_lot::{MappedMutexGuard, MutexGuard};
use std::ops::Deref;

/// A read only Mapped Mutex guard.
///
/// In the bounds of Beetle, sometimes a mapped Mutex guard reference is needed in order to
/// get a reference to a value that is normally contained within a Mutex'd value. However,
/// this allows mutable access to these values, which can conflict with the event
/// system that Beetle uses. To solve this problem, this structure simply wraps
/// the MappedMutexGuard, but only derefs to &T.
///
/// # Example
///
/// ```
/// use beetle::ReadOnlyMappedMutexGuard;
/// use parking_lot::{Mutex, MutexGuard, MappedMutexGuard};
///
/// struct Foobar {
///     foo: String,
/// }
///
/// struct FoobarWrapper {
///     inner: Mutex<Foobar>,
/// }
///
/// impl FoobarWrapper {
///     fn get_foo(&self) -> ReadOnlyMappedMutexGuard<'_, str> {
///         let mutable_guard = MutexGuard::map(self.inner.lock(), |i| &i.foo);
///         ReadOnlyMappedMutexGuard::new(mutable_guard)
///     }
/// }
///
/// fn main() {
///     let f = Foobar { foo: "Hello world!".to_string() };
///     let fw = FoobarWrapper { inner: Mutex::new(f) };
///     
///     let the_foo = String::from(*fw.get_foo());
///     assert_eq!(the_foo, String::from("Hello world!"));
/// }
/// ```
pub struct ReadOnlyMappedMutexGuard<'a, T: ?Sized>(MappedMutexGuard<'a, T>);

impl<'a, T: ?Sized + 'a> Deref for ReadOnlyMappedMutexGuard<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<'a, T: ?Sized + 'a> ReadOnlyMappedMutexGuard<'a, T> {
    /// Create a new ReadOnlyMappedMutexGuard from a MappedMutexGuard.
    #[inline]
    pub fn new(inner: MappedMutexGuard<'a, T>) -> Self {
        Self(inner)
    }

    /// Convert this mutex guard to its inner mutable mutex guard.
    ///
    /// # Safety
    ///
    /// This is considered unsafe. Beetle is built on the assumption that a changed property
    /// will correspond to a changed event. Therefore, getting the inner mutable reference
    /// and mutating the property without the proper channels will cause undefined behavior.
    ///
    /// It is strongly recommended to use the Beetle objects' natural set_X methods, as they
    /// emit the proper events needed.
    #[inline]
    pub unsafe fn into_inner(self) -> MappedMutexGuard<'a, T> {
        self.0
    }

    /// Create a read only mutex guard from an existing MutexGuard and a function
    /// for mapping it. This is shorthand for `ReadOnlyMappedMutexGuard::new(MutexGuard::map(lock, f))`.
    #[inline]
    pub fn from_guard<U: ?Sized, F>(guard: MutexGuard<'a, U>, f: F) -> Self
    where
        F: FnOnce(&mut U) -> &mut T,
    {
        ReadOnlyMappedMutexGuard::new(MutexGuard::map(guard, f))
    }
}
