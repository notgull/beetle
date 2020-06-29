/* -----------------------------------------------------------------------------------
 * src/keyboard/mod.rs - Enums and structs representing key presses.
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

use std::convert::TryInto;

/// The types of keys that can be depressed on the keyboard.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum KeyType {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
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
    F21,
    F22,
    F23,
    F24,
    Accept,
    Add,
    Again,
    AllCandidates,
    Alphanumeric,
    AltGraph,
    /// The & key
    Ampersand,
    /// The * key
    Asterisk,
    /// The @ key
    At,
    LeftAlt,
    RightAlt,
    BackQuote,
    /// The \ Key
    BackSlash,
    BackSpace,
    /// The | key
    Bar,
    Begin,
    LeftBrace,
    RightBrace,
    Cancel,
    CapsLock,
    /// The ^ key
    Circumflex,
    Clear,
    LeftBracket,
    RightBracket,
    CodeInput,
    Colon,
    Comma,
    Compose,
    ContextMenu,
    LeftControl,
    RightControl,
    Convert,
    /// Function key Copy
    FCopy,
    Cut,
    Decimal,
    Delete,
    Divide,
    /// The $ key
    Dollar,
    End,
    Enter,
    /// The = key
    Equals,
    Escape,
    /// The € key
    EuroSign,
    /// The ! key
    ExclamationMark,
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
    /// The ¡ key
    InvertedExclamationMark,
    JapaneseHiragana,
    JapaneseKatakana,
    JapaneseRoman,
    Kana,
    KanaLock,
    Kanji,
    Katakana,
    KeypadUp,
    KeypadDown,
    KeypadRight,
    KeypadLeft,
    LeftParenthesis,
    RightParenthesis,
    Less,
    Meta,
    Minus,
    ModeChange,
    Multiply,
    DontConvert,
    NumLock,
    /// The # key
    NumberSign,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    PageDown,
    PageUp,
    Paste,
    Pause,
    /// The % key
    Percent,
    /// The . key
    Period,
    /// The + key
    Plus,
    PreviousCandidate,
    PrintScreen,
    Props,
    /// The ? key
    QuestionMark,
    Quote,
    DoubleQuote,
    RomanCharacters,
    ScrollLock,
    /// The ; key
    Semicolon,
    Separator,
    LeftShift,
    RightShift,
    /// The / key
    Slash,
    Space,
    Stop,
    Subtract,
    Tab,
    /// The ~ key
    Tilde,
    /// The _ key
    Underscore,
    Undo,
    Windows,
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

impl Default for KeyType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// A key being pressed or released.
#[derive(Debug, Default)]
pub struct KeyInfo {
    ty: KeyType,
    is_ctrl: bool,
    is_alt: bool,
    is_shift: bool,
    is_alt_graph: bool,
    is_button1: bool,
    is_button2: bool,
    is_button3: bool,
    is_meta: bool,
}

impl KeyInfo {
    /// Create a new key info using a key code.
    #[inline]
    pub fn new(ki: KeyType) -> KeyInfo {
        Self {
            ty: ki,
            ..Default::default()
        }
    }

    /// Get the key code.
    #[inline]
    pub fn key_type(&self) -> KeyType {
        self.ty
    }

    /// Set the key code.
    #[inline]
    pub fn set_key_type(&mut self, ki: KeyType) {
        self.ty = ki;
    }

    /// Is the control key pressed?
    #[inline]
    pub fn ctrl(&self) -> bool {
        self.is_ctrl
    }

    /// Set whether the control key is pressed.
    #[inline]
    pub fn set_ctrl(&mut self, is_ctrl: bool) {
        self.is_ctrl = is_ctrl;
    }

    /// Is the alt key pressed?
    #[inline]
    pub fn alt(&self) -> bool {
        self.is_alt
    }

    /// Set whether the alt key is pressed.
    #[inline]
    pub fn set_alt(&mut self, is_alt: bool) {
        self.is_alt = is_alt;
    }

    /// Is the shift key pressed?
    #[inline]
    pub fn shift(&self) -> bool {
        self.is_shift
    }

    /// Set whether the shift key is pressed.
    #[inline]
    pub fn set_shift(&mut self, is_shift: bool) {
        self.is_shift = is_shift;
    }

    /// Is the alt graph key pressed?
    #[inline]
    pub fn alt_graph(&self) -> bool {
        self.is_alt_graph
    }

    /// Set whether the alt graph key is pressed.
    #[inline]
    pub fn set_alt_graph(&mut self, is_alt_graph: bool) {
        self.is_alt_graph = is_alt_graph;
    }

    /// Is the first mouse button pressed?
    #[inline]
    pub fn button1(&self) -> bool {
        self.is_button1
    }

    /// Set whether the first mouse button is pressed.
    #[inline]
    pub fn set_button1(&mut self, is_button1: bool) {
        self.is_button1 = is_button1;
    }

    /// Is the second mouse button pressed?
    #[inline]
    pub fn button2(&self) -> bool {
        self.is_button2
    }

    /// Set whether the second mouse button is pressed.
    #[inline]
    pub fn set_button2(&mut self, is_button2: bool) {
        self.is_button2 = is_button2;
    }

    /// Is the third mouse button pressed?
    #[inline]
    pub fn button3(&self) -> bool {
        self.is_button3
    }

    /// Set whether the third mouse button is pressed.
    #[inline]
    pub fn set_button3(&mut self, is_button3: bool) {
        self.is_button3 = is_button3;
    }

    /// Is the meta button pressed?
    #[inline]
    pub fn meta(&self) -> bool {
        self.is_meta
    }

    /// Set whether the meta button is pressed.
    #[inline]
    pub fn set_meta(&mut self, is_meta: bool) {
        self.is_meta = is_meta;
    }
}

mod x11_keysym_table;

#[cfg(target_os = "linux")]
impl KeyType {
    /// Convert an X11 keysym to a key type.
    #[inline]
    pub fn from_keysym(ks: flutterbug::KeySym) -> KeyType {
        let u: usize = ks
            .try_into()
            .expect("Unable to convert KeySym into array index");
        if u >= x11_keysym_table::X11_KEYSYM_TABLE.len() {
            KeyType::Unknown
        } else {
            x11_keysym_table::X11_KEYSYM_TABLE[u]
        }
    }
}

mod win32_keysym_table;

#[cfg(windows)]
impl KeyType {
    /// Convert a Win32 virtual keycode to a key type.
    #[inline]
    pub fn from_vk(vk: usize) -> KeyType {
        if vk >= win32_keysym_table::WIN32_KEYSYM_TABLE.len() {
            KeyType::Unknown
        } else {
            win32_keysym_table::WIN32_KEYSYM_TABLE[vk]
        }
    }
}
