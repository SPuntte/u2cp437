// This crate is derived from https://github.com/nabijaczleweli/codepage-437, which is licenced as
// follows:
//
// The MIT License (MIT)
//
// Copyright (c) 2018 nabijaczleweli
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Bare-bones Unicode to [code page 437](https://en.wikipedia.org/wiki/Code_page_437) conversion.
//!
//! Derived for `#![no_std]` use from [`nabijaczleweli/codepage-437`](https://github.com/nabijaczleweli/codepage-437).

#![no_std]

/// Do it.
///
/// ```rust
/// // Matching unicode code points produce code page 437 code points as `u8`
/// assert_eq!(u2cp437::convert('☻'), Some(0x02));
///
/// // Certain CP437 code points have multiple unicode equivalents
/// let mu1 = u2cp437::convert('\u{0000B5}'); // "MICRO SIGN"
/// let mu2 = u2cp437::convert('\u{0003BC}'); // "GREEK SMALL LETTER MU"
/// assert_eq!(mu1, Some(0xE6));
/// assert_eq!(mu1, mu2);
///
/// // Unicode code points that have no equivalent in CP437, including all control characters but
/// // the null character, produce nothing
/// assert_eq!(u2cp437::convert('🤔'), None);
/// assert_eq!(u2cp437::convert('\n'), None);
/// assert_eq!(u2cp437::convert('\0'), Some(0x0));
/// ```
pub fn convert(unicode: char) -> Option<u8> {
    Some(match unicode {
        // Non-overlapping code points
        '\u{00263A}' => 0x01, // WHITE SMILING FACE
        '\u{00263B}' => 0x02, // BLACK SMILING FACE
        '\u{002665}' => 0x03, // BLACK HEART SUIT
        '\u{002666}' => 0x04, // BLACK DIAMOND SUIT
        '\u{002663}' => 0x05, // BLACK CLUB SUIT
        '\u{002660}' => 0x06, // BLACK SPADE SUIT
        '\u{002022}' => 0x07, // BULLET
        '\u{0025D8}' => 0x08, // INVERSE BULLET
        '\u{0025CB}' => 0x09, // WHITE CIRCLE
        '\u{0025D9}' => 0x0A, // INVERSE WHITE CIRCLE
        '\u{002642}' => 0x0B, // MALE SIGN
        '\u{002640}' => 0x0C, // FEMALE SIGN
        '\u{00266A}' => 0x0D, // EIGHTH NOTE
        '\u{00266B}' => 0x0E, // BEAMED EIGHTH NOTES
        '\u{00263C}' => 0x0F, // WHITE SUN WITH RAYS
        '\u{0025BA}' => 0x10, // BLACK RIGHT-POINTING POINTER
        '\u{0025C4}' => 0x11, // BLACK LEFT-POINTING POINTER
        '\u{002195}' => 0x12, // UP DOWN ARROW
        '\u{00203C}' => 0x13, // DOUBLE EXCLAMATION MARK
        '\u{0000B6}' => 0x14, // PILCROW SIGN
        '\u{0000A7}' => 0x15, // SECTION SIGN
        '\u{0025AC}' => 0x16, // BLACK RECTANGLE
        '\u{0021A8}' => 0x17, // UP DOWN ARROW WITH BASE
        '\u{002191}' => 0x18, // UPWARDS ARROW
        '\u{002193}' => 0x19, // DOWNWARDS ARROW
        '\u{002192}' => 0x1A, // RIGHTWARDS ARROW
        '\u{002190}' => 0x1B, // LEFTWARDS ARROW
        '\u{00221F}' => 0x1C, // RIGHT ANGLE
        '\u{002194}' => 0x1D, // LEFT RIGHT ARROW
        '\u{0025B2}' => 0x1E, // BLACK UP-POINTING TRIANGLE
        '\u{0025BC}' => 0x1F, // BLACK DOWN-POINTING TRIANGLE
        '\u{002302}' => 0x7F, // HOUSE
        '\u{0000C7}' => 0x80, // LATIN CAPITAL LETTER C WITH CEDILLA
        '\u{0000FC}' => 0x81, // LATIN SMALL LETTER U WITH DIAERESIS
        '\u{0000E9}' => 0x82, // LATIN SMALL LETTER E WITH ACUTE
        '\u{0000E2}' => 0x83, // LATIN SMALL LETTER A WITH CIRCUMFLEX
        '\u{0000E4}' => 0x84, // LATIN SMALL LETTER A WITH DIAERESIS
        '\u{0000E0}' => 0x85, // LATIN SMALL LETTER A WITH GRAVE
        '\u{0000E5}' => 0x86, // LATIN SMALL LETTER A WITH RING ABOVE
        '\u{0000E7}' => 0x87, // LATIN SMALL LETTER C WITH CEDILLA
        '\u{0000EA}' => 0x88, // LATIN SMALL LETTER E WITH CIRCUMFLEX
        '\u{0000EB}' => 0x89, // LATIN SMALL LETTER E WITH DIAERESIS
        '\u{0000E8}' => 0x8A, // LATIN SMALL LETTER E WITH GRAVE
        '\u{0000EF}' => 0x8B, // LATIN SMALL LETTER I WITH DIAERESIS
        '\u{0000EE}' => 0x8C, // LATIN SMALL LETTER I WITH CIRCUMFLEX
        '\u{0000EC}' => 0x8D, // LATIN SMALL LETTER I WITH GRAVE
        '\u{0000C4}' => 0x8E, // LATIN CAPITAL LETTER A WITH DIAERESIS
        '\u{0000C5}' => 0x8F, // LATIN CAPITAL LETTER A WITH RING ABOVE
        '\u{0000C9}' => 0x90, // LATIN CAPITAL LETTER E WITH ACUTE
        '\u{0000E6}' => 0x91, // LATIN SMALL LETTER AE
        '\u{0000C6}' => 0x92, // LATIN CAPITAL LETTER AE
        '\u{0000F4}' => 0x93, // LATIN SMALL LETTER O WITH CIRCUMFLEX
        '\u{0000F6}' => 0x94, // LATIN SMALL LETTER O WITH DIAERESIS
        '\u{0000F2}' => 0x95, // LATIN SMALL LETTER O WITH GRAVE
        '\u{0000FB}' => 0x96, // LATIN SMALL LETTER U WITH CIRCUMFLEX
        '\u{0000F9}' => 0x97, // LATIN SMALL LETTER U WITH GRAVE
        '\u{0000FF}' => 0x98, // LATIN SMALL LETTER Y WITH DIAERESIS
        '\u{0000D6}' => 0x99, // LATIN CAPITAL LETTER O WITH DIAERESIS
        '\u{0000DC}' => 0x9A, // LATIN CAPITAL LETTER U WITH DIAERESIS
        '\u{0000A2}' => 0x9B, // CENT SIGN
        '\u{0000A3}' => 0x9C, // POUND SIGN
        '\u{0000A5}' => 0x9D, // YEN SIGN
        '\u{0020A7}' => 0x9E, // PESETA SIGN
        '\u{000192}' => 0x9F, // LATIN SMALL LETTER F WITH HOOK
        '\u{0000E1}' => 0xA0, // LATIN SMALL LETTER A WITH ACUTE
        '\u{0000ED}' => 0xA1, // LATIN SMALL LETTER I WITH ACUTE
        '\u{0000F3}' => 0xA2, // LATIN SMALL LETTER O WITH ACUTE
        '\u{0000FA}' => 0xA3, // LATIN SMALL LETTER U WITH ACUTE
        '\u{0000F1}' => 0xA4, // LATIN SMALL LETTER N WITH TILDE
        '\u{0000D1}' => 0xA5, // LATIN CAPITAL LETTER N WITH TILDE
        '\u{0000AA}' => 0xA6, // FEMININE ORDINAL INDICATOR
        '\u{0000BA}' => 0xA7, // MASCULINE ORDINAL INDICATOR
        '\u{0000BF}' => 0xA8, // INVERTED QUESTION MARK
        '\u{002310}' => 0xA9, // REVERSED NOT SIGN
        '\u{0000AC}' => 0xAA, // NOT SIGN
        '\u{0000BD}' => 0xAB, // VULGAR FRACTION ONE HALF
        '\u{0000BC}' => 0xAC, // VULGAR FRACTION ONE QUARTER
        '\u{0000A1}' => 0xAD, // INVERTED EXCLAMATION MARK
        '\u{0000AB}' => 0xAE, // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
        '\u{0000BB}' => 0xAF, // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
        '\u{002591}' => 0xB0, // LIGHT SHADE
        '\u{002592}' => 0xB1, // MEDIUM SHADE
        '\u{002593}' => 0xB2, // DARK SHADE
        '\u{002502}' => 0xB3, // BOX DRAWINGS LIGHT VERTICAL
        '\u{002524}' => 0xB4, // BOX DRAWINGS LIGHT VERTICAL AND LEFT
        '\u{002561}' => 0xB5, // BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE
        '\u{002562}' => 0xB6, // BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE
        '\u{002556}' => 0xB7, // BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE
        '\u{002555}' => 0xB8, // BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE
        '\u{002563}' => 0xB9, // BOX DRAWINGS DOUBLE VERTICAL AND LEFT
        '\u{002551}' => 0xBA, // BOX DRAWINGS DOUBLE VERTICAL
        '\u{002557}' => 0xBB, // BOX DRAWINGS DOUBLE DOWN AND LEFT
        '\u{00255D}' => 0xBC, // BOX DRAWINGS DOUBLE UP AND LEFT
        '\u{00255C}' => 0xBD, // BOX DRAWINGS UP DOUBLE AND LEFT SINGLE
        '\u{00255B}' => 0xBE, // BOX DRAWINGS UP SINGLE AND LEFT DOUBLE
        '\u{002510}' => 0xBF, // BOX DRAWINGS LIGHT DOWN AND LEFT
        '\u{002514}' => 0xC0, // BOX DRAWINGS LIGHT UP AND RIGHT
        '\u{002534}' => 0xC1, // BOX DRAWINGS LIGHT UP AND HORIZONTAL
        '\u{00252C}' => 0xC2, // BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
        '\u{00251C}' => 0xC3, // BOX DRAWINGS LIGHT VERTICAL AND RIGHT
        '\u{002500}' => 0xC4, // BOX DRAWINGS LIGHT HORIZONTAL
        '\u{00253C}' => 0xC5, // BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
        '\u{00255E}' => 0xC6, // BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE
        '\u{00255F}' => 0xC7, // BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE
        '\u{00255A}' => 0xC8, // BOX DRAWINGS DOUBLE UP AND RIGHT
        '\u{002554}' => 0xC9, // BOX DRAWINGS DOUBLE DOWN AND RIGHT
        '\u{002569}' => 0xCA, // BOX DRAWINGS DOUBLE UP AND HORIZONTAL
        '\u{002566}' => 0xCB, // BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
        '\u{002560}' => 0xCC, // BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
        '\u{002550}' => 0xCD, // BOX DRAWINGS DOUBLE HORIZONTAL
        '\u{00256C}' => 0xCE, // BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
        '\u{002567}' => 0xCF, // BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE
        '\u{002568}' => 0xD0, // BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE
        '\u{002564}' => 0xD1, // BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE
        '\u{002565}' => 0xD2, // BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE
        '\u{002559}' => 0xD3, // BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE
        '\u{002558}' => 0xD4, // BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE
        '\u{002552}' => 0xD5, // BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE
        '\u{002553}' => 0xD6, // BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE
        '\u{00256B}' => 0xD7, // BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE
        '\u{00256A}' => 0xD8, // BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE
        '\u{002518}' => 0xD9, // BOX DRAWINGS LIGHT UP AND LEFT
        '\u{00250C}' => 0xDA, // BOX DRAWINGS LIGHT DOWN AND RIGHT
        '\u{002588}' => 0xDB, // FULL BLOCK
        '\u{002584}' => 0xDC, // LOWER HALF BLOCK
        '\u{00258C}' => 0xDD, // LEFT HALF BLOCK
        '\u{002590}' => 0xDE, // RIGHT HALF BLOCK
        '\u{002580}' => 0xDF, // UPPER HALF BLOCK
        '\u{0003B1}' => 0xE0, // GREEK SMALL LETTER ALPHA
        '\u{0000DF}' => 0xE1, // LATIN SMALL LETTER SHARP S
        '\u{000393}' => 0xE2, // GREEK CAPITAL LETTER GAMMA
        '\u{0003C0}' => 0xE3, // GREEK SMALL LETTER PI
        '\u{0003A3}' => 0xE4, // GREEK CAPITAL LETTER SIGMA
        '\u{0003C3}' => 0xE5, // GREEK SMALL LETTER SIGMA
        '\u{0000B5}' => 0xE6, // MICRO SIGN
        '\u{0003C4}' => 0xE7, // GREEK SMALL LETTER TAU
        '\u{0003A6}' => 0xE8, // GREEK CAPITAL LETTER PHI
        '\u{000398}' => 0xE9, // GREEK CAPITAL LETTER THETA
        '\u{0003A9}' => 0xEA, // GREEK CAPITAL LETTER OMEGA
        '\u{0003B4}' => 0xEB, // GREEK SMALL LETTER DELTA
        '\u{00221E}' => 0xEC, // INFINITY
        '\u{0003C6}' => 0xED, // GREEK SMALL LETTER PHI
        '\u{0003B5}' => 0xEE, // GREEK SMALL LETTER EPSILON
        '\u{002229}' => 0xEF, // INTERSECTION
        '\u{002261}' => 0xF0, // IDENTICAL TO
        '\u{0000B1}' => 0xF1, // PLUS-MINUS SIGN
        '\u{002265}' => 0xF2, // GREATER-THAN OR EQUAL TO
        '\u{002264}' => 0xF3, // LESS-THAN OR EQUAL TO
        '\u{002320}' => 0xF4, // TOP HALF INTEGRAL
        '\u{002321}' => 0xF5, // BOTTOM HALF INTEGRAL
        '\u{0000F7}' => 0xF6, // DIVISION SIGN
        '\u{002248}' => 0xF7, // ALMOST EQUAL TO
        '\u{0000B0}' => 0xF8, // DEGREE SIGN
        '\u{002219}' => 0xF9, // BULLET OPERATOR
        '\u{0000B7}' => 0xFA, // MIDDLE DOT
        '\u{00221A}' => 0xFB, // SQUARE ROOT
        '\u{00207F}' => 0xFC, // SUPERSCRIPT LATIN SMALL LETTER N
        '\u{0000B2}' => 0xFD, // SUPERSCRIPT TWO
        '\u{0025A0}' => 0xFE, // BLACK SQUARE
        '\u{0000A0}' => 0xFF, // NO-BREAK SPACE

        // Variants
        '\u{0000A6}' => 0x7C, // BROKEN BAR
        '\u{000394}' => 0x7F, // Greek capital delta
        '\u{0003B2}' => 0xE1, // Greek small beta
        '\u{0003A0}' => 0xE3, // Greek capital pi
        '\u{00220F}' => 0xE3, // n-ary product sign
        '\u{002211}' => 0xE4, // n-ary summation sign
        '\u{0003BC}' => 0xE6, // Mu Small
        '\u{0000F0}' => 0xEB, // small eth
        '\u{002202}' => 0xEB, // partial derivative sign
        '\u{0003D5}' => 0xED, // Phi Small (Closed Form)
        '\u{01D719}' => 0xED, // Italicized Phi Small (Closed Form)
        '\u{002205}' => 0xED, // empty set sign
        '\u{002300}' => 0xED, // diameter sign
        '\u{0000D8}' => 0xED, // Capital Latin letter O with stroke
        '\u{0000F8}' => 0xED, // Lowercase Latin letter O with stroke
        '\u{002208}' => 0xEE, // element-of sign
        '\u{0020AC}' => 0xEE, // euro sign
        '\u{002713}' => 0xFB, // check mark
        '\u{002007}' => 0xFF, // FIGURE SPACE
        '\u{00202F}' => 0xFF, // NARROW NO-BREAK SPACE

        c => {
            // Overlapping code points
            if c == '\u{00}' || (c > '\u{1F}' && c < '\u{7F}') {
                c as u8
            } else {
                return None;
            }
        }
    })
}
