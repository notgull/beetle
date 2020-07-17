/* -----------------------------------------------------------------------------------
 * src/instance/internal/mod.rs - Internal interface used by the Instance.
 * beetle - Pull-based GUI framework
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

use super::{Instance, InstanceType};
use crate::{Event, Pixel, Texture, Window};
use alloc::{collections::VecDeque, string::String};
use euclid::Rect;
use smallvec::SmallVec;

#[cfg(target_os = "linux")]
pub mod flutter;
#[cfg(windows)]
pub mod porc;

/// Internal interface used by the instance.
pub trait GenericInternalInstance {
    fn create_window(
        &self,
        parent: Option<&Window>,
        text: String,
        bounds: Rect<u32, Pixel>,
        background: Option<Texture>,
        instance_ref: Instance,
    ) -> crate::Result<Window>;

    fn hold_for_events(
        &self,
        output: &mut SmallVec<[Event; 8]>,
        inst: &Instance,
    ) -> crate::Result<()>;

    #[inline]
    fn needs_quit(&self) -> bool {
        false
    }
}

/// Storage for the internal instance;
pub enum InternalInstance {
    #[cfg(windows)]
    Porc(porc::PorcII),
    #[cfg(target_os = "linux")]
    Flutter(flutter::FlutterII),
}

impl InternalInstance {
    pub fn generic(&self) -> &dyn GenericInternalInstance {
        match self {
            #[cfg(windows)]
            Self::Porc(ref p) => p,
            #[cfg(target_os = "linux")]
            Self::Flutter(ref f) => f,
        }
    }

    pub fn ty(&self) -> InstanceType {
        match self {
            #[cfg(windows)]
            Self::Porc(_p) => InstanceType::Porcupine,
            #[cfg(target_os = "linux")]
            Self::Flutter(_f) => InstanceType::Flutterbug,
        }
    }
}
