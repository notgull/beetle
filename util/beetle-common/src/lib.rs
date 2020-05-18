/* -----------------------------------------------------------------------------------
 * util/beetle-common/src/lib.rs - Common items for all Beetle crates.
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

/// Represents a key on the keyboard.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Key {
    K_0,
    K_1,
    K_2,
    K_3,
    K_4,
    K_5,
    K_6,
    K_7,
    K_8,
    K_9,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    Accept,
    Add,
    Again,
    AllCandidates,
    Alphanumeric,
    Alt,
    AltGraph,
    Ampersand,
    Asterisk,
    At,
    BackQuote,
    Backslash,
    Backspace,
    Begin,
    BraceLeft,
    BraceRight,
    Cancel,
    CapsLock,
    Circumflex,
    Clear,
    CloseBracket,
    CodeInput,
    Colon,
    Comma,
    Compose,
    ContextMenu,
    Control,
    Convert,
    K_Copy,
    Cut,
    DeadAboveDot,
    DeadAboveRing,
    DeadAcute,
    DeadBreve,
    DeadCaron,
    DeadCedilla,
    DeadCircumflex,
    DeadDiaresis,
    DeadDoubleacute,
    DeadGrave,
    DeadIota,
    DeadMacron,
    DeadOgonek,
    DeadSemivoicedSound,
    DeadTilde,
    DeadVoicedSound,
    Decimal,
    Delete,
    Divide,
    Dollar,
    Down,
    End,
    Enter,
    Equals,
    Escape,
    EuroSign,
    ExclamationMark,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Final,
    Find,
    FullWidth,
    Greater,
    HalfWidth,
    Help,
    Hiragana,
    Home,
    InputMethodOnOff,
    Insert,
    InvertedExclamationMark,
    JapaneseHiragana,
    JapaneseKatakana,
    Kana,
    KanaLock,
    Kanji,
    Katakana,
}
