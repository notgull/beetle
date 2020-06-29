/* -----------------------------------------------------------------------------------
 * src/keyboard/x11_keysym_table.rs - X11 key symbol table.
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


#![cfg(target_os = "linux")]

use super::KeyType::{self, *};

#[allow(non_upper_case_globals)]
const Un: KeyType = Unknown;

// table of x11 keysyms to beetle keycodes
pub const X11_KEYSYM_TABLE: [KeyType; 0xAF] = [
    // the first 0x1F numbers are unused
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Space,            // 0x20 = XK_Space
    ExclamationMark,  // 0x21 = XK_exclam
    DoubleQuote,      // 0x22 = XK_quotedbl
    NumberSign,       // 0x23 = XK_numbersign
    Dollar,           // 0x24 = XK_dollar
    Percent,          // 0x25 = XK_percent
    Ampersand,        // 0x26 = XK_ampersand,
    Quote,            // 0x27 = XK_apostrophe
    LeftParenthesis,  // 0x28 = XK_parenleft
    RightParenthesis, // 0x29 = XK_parenright
    Asterisk,         // 0x2a = XK_asterisk
    Plus,             // 0x2b = XK_plus
    Comma,            // 0x2c = XK_comma,
    Minus,            // 0x2d = XK_minus
    Period,           // 0x2e = XK_period,
    Slash,            // 0x2f = XK_Slash,
    // number keys
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
    Colon,        // 0x3a = XK_colon
    Semicolon,    // 0x3b = XK_semicolon
    Less,         // 0x3c = XK_less
    Equals,       // 0x3d = XK_equal
    Greater,      // 0x3e = XK_greater,
    QuestionMark, // 0x3f = XK_question,
    At,           // 0x40 = XK_at,
    // the alphabet
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,  // 0x5b = XK_bracketleft,
    BackSlash,    // 0x5c = XK_backslash
    RightBracket, // 0x5d = XK_bracketright
    Circumflex,   // 0x5e = XK_asciicircum
    Underscore,   // 0x5f = XK_underscore
    BackQuote,    // 0x60 = XK_grave
    // the alphabet, again
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBrace,  // 0x7b = XK_braceleft
    Bar,        // 0x7c = XK_bar
    RightBrace, // 0x7d = XK_braceright
    Tilde,      // 0x7e = XK_tidle
    // 0x7f to 0x9f are unused
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Space,                   // 0xa0 = XK_nobreakspace
    InvertedExclamationMark, // 0xa1 = XK_exclamdown
    // TODO: this block contains characters that should be filled out
    // such as shift, caps lock, etc
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    Un,
    //        Hyphen, // 0xad = XK_hyphen
];
