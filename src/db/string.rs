//! Pkmnapi string module
//!
//! # Example
//!
//! ```
//! use pkmnapi::db::string::*;
//!
//! let string = PkmnapiDBString::new(&[0x80, 0x81, 0x82, 0x50]);
//!
//! assert_eq!(string.value, vec![0x80, 0x81, 0x82, 0x50]);
//! ```

use std::fmt;

/// Representation of a ROM string
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBString {
    pub value: Vec<u8>,
}

impl PkmnapiDBString {
    /// Creates a new ROM string from an array of bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    ///
    /// let string = PkmnapiDBString::new(&[0x80, 0x81, 0x82, 0x50]);
    ///
    /// assert_eq!(string.value, vec![0x80, 0x81, 0x82, 0x50]);
    /// ```
    pub fn new(dbstr: &[u8]) -> PkmnapiDBString {
        let value = dbstr.to_vec();

        PkmnapiDBString { value }
    }

    /// Creates a ROM string from a &str or String
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    ///
    /// let string = PkmnapiDBString::from("ABC@");
    ///
    /// assert_eq!(string.value, vec![0x80, 0x81, 0x82, 0x50]);
    /// ```
    pub fn from<S: Into<String>>(dbstr: S) -> Self {
        let dbstr = dbstr.into();
        let mut chars = dbstr.chars().peekable();
        let mut value: Vec<u8> = vec![];

        while let Some(c) = chars.next() {
            match c {
                '\'' => {
                    let next_c = chars.peek().unwrap();

                    match next_c {
                        's' => {
                            value.push(0xBD);
                            chars.next();

                            continue;
                        }
                        't' => {
                            value.push(0xBE);
                            chars.next();

                            continue;
                        }
                        _ => {}
                    };
                }
                _ => {}
            };

            let chr = match c {
                '¶' => 0x49,
                '\n' => 0x4E,
                '@' => 0x50,
                '#' => 0x54,
                ' ' => 0x7F,
                'A' => 0x80,
                'B' => 0x81,
                'C' => 0x82,
                'D' => 0x83,
                'E' => 0x84,
                'F' => 0x85,
                'G' => 0x86,
                'H' => 0x87,
                'I' => 0x88,
                'J' => 0x89,
                'K' => 0x8a,
                'L' => 0x8b,
                'M' => 0x8c,
                'N' => 0x8d,
                'O' => 0x8e,
                'P' => 0x8f,
                'Q' => 0x90,
                'R' => 0x91,
                'S' => 0x92,
                'T' => 0x93,
                'U' => 0x94,
                'V' => 0x95,
                'W' => 0x96,
                'X' => 0x97,
                'Y' => 0x98,
                'Z' => 0x99,
                'a' => 0xA0,
                'b' => 0xA1,
                'c' => 0xA2,
                'd' => 0xA3,
                'e' => 0xA4,
                'f' => 0xA5,
                'g' => 0xA6,
                'h' => 0xA7,
                'i' => 0xA8,
                'j' => 0xA9,
                'k' => 0xAA,
                'l' => 0xAB,
                'm' => 0xAC,
                'n' => 0xAD,
                'o' => 0xAE,
                'p' => 0xAF,
                'q' => 0xB0,
                'r' => 0xB1,
                's' => 0xB2,
                't' => 0xB3,
                'u' => 0xB4,
                'v' => 0xB5,
                'w' => 0xB6,
                'x' => 0xB7,
                'y' => 0xB8,
                'z' => 0xB9,
                '\'' => 0xE0,
                '-' => 0xE3,
                '.' => 0xE8,
                '♂' => 0xEF,
                ',' => 0xF4,
                '♀' => 0xF5,
                '0' => 0xF6,
                '1' => 0xF7,
                '2' => 0xF8,
                '3' => 0xF9,
                '4' => 0xFA,
                '5' => 0xFB,
                '6' => 0xFC,
                '7' => 0xFD,
                '8' => 0xFE,
                '9' => 0xFF,
                _ => c as u8,
            };

            value.push(chr);
        }

        PkmnapiDBString { value }
    }
}

impl fmt::Display for PkmnapiDBString {
    /// Converts the internal string represnetation to a String
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    ///
    /// let string = PkmnapiDBString::new(&[0x80, 0x81, 0x82, 0x50]);
    /// let decoded = string.to_string();
    ///
    /// assert_eq!(decoded, "ABC");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: String = self
            .value
            .iter()
            .map(|c| match c {
                0x49 => "¶",
                0x4E => "\n",
                0x50 => "@",
                0x54 => "#",
                0x7F => " ",
                0x80 => "A",
                0x81 => "B",
                0x82 => "C",
                0x83 => "D",
                0x84 => "E",
                0x85 => "F",
                0x86 => "G",
                0x87 => "H",
                0x88 => "I",
                0x89 => "J",
                0x8a => "K",
                0x8b => "L",
                0x8c => "M",
                0x8d => "N",
                0x8e => "O",
                0x8f => "P",
                0x90 => "Q",
                0x91 => "R",
                0x92 => "S",
                0x93 => "T",
                0x94 => "U",
                0x95 => "V",
                0x96 => "W",
                0x97 => "X",
                0x98 => "Y",
                0x99 => "Z",
                0xA0 => "a",
                0xA1 => "b",
                0xA2 => "c",
                0xA3 => "d",
                0xA4 => "e",
                0xA5 => "f",
                0xA6 => "g",
                0xA7 => "h",
                0xA8 => "i",
                0xA9 => "j",
                0xAA => "k",
                0xAB => "l",
                0xAC => "m",
                0xAD => "n",
                0xAE => "o",
                0xAF => "p",
                0xB0 => "q",
                0xB1 => "r",
                0xB2 => "s",
                0xB3 => "t",
                0xB4 => "u",
                0xB5 => "v",
                0xB6 => "w",
                0xB7 => "x",
                0xB8 => "y",
                0xB9 => "z",
                0xBD => "'s",
                0xBE => "'t",
                0xE0 => "'",
                0xE3 => "-",
                0xE8 => ".",
                0xEF => "♂",
                0xF4 => ",",
                0xF5 => "♀",
                0xF6 => "0",
                0xF7 => "1",
                0xF8 => "2",
                0xF9 => "3",
                0xFA => "4",
                0xFB => "5",
                0xFC => "6",
                0xFD => "7",
                0xFE => "8",
                0xFF => "9",
                _ => "?",
            })
            .collect();

        let at_offset = value.find('@').unwrap_or(value.len());
        let mut value = String::from(&value);

        value.truncate(at_offset);

        write!(f, "{}", value)
    }
}
