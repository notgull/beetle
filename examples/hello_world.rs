/* -----------------------------------------------------------------------------------
 * examples/hello_world.rs - Basic opening of a window.
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

use beetle::{EventType, Instance, Result};
use euclid::rect;

fn main() -> Result<()> {
    env_logger::init();

    let instance = Instance::new()?;
    let window = instance.create_window(
        None,
        "Hello world!".to_string(),
        rect(0, 0, 400, 200),
        None,
        true,
    )?;

    window.receive_events(&[EventType::MouseButtonDown, EventType::KeyDown])?;
    window.show()?; 

    'evloop: loop {
        let event = instance.next_event()?;

        // do something if this is a mouse event
        match event.ty() {
            EventType::MouseButtonDown => {
                let coords = event.click_location().unwrap();
                println!("Mouse click at ({}, {})", coords.x, coords.y);
            }
            EventType::KeyDown => {
                let key = event.key().unwrap();
                println!("Key Information: {:?}", key);
            }
            _ => ()
        }

        // if this is a quit event, end the event loop
        // otherwise, dispatch it to its intended target
        if event.is_exit_event() {
            break 'evloop;
        } else {
            event.dispatch()?;
        }
    }

    Ok(())
}
