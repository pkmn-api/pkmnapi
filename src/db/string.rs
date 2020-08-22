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
        PkmnapiDBString {
            value: dbstr.to_vec(),
        }
    }

    /// Creates a ROM string from a &str or String
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    ///
    /// let string = PkmnapiDBString::from_string("ABC@");
    ///
    /// assert_eq!(string.value, vec![0x80, 0x81, 0x82, 0x50]);
    /// ```
    pub fn from_string<S: Into<String>>(dbstr: S) -> Self {
        let value: Vec<u8> = dbstr
            .into()
            .chars()
            .map(|c| match c {
                '@' => 0x50,
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
                '\'' => 0xE0,
                '-' => 0xE3,
                '.' => 0xE8,
                '♂' => 0xEF,
                '♀' => 0xF5,
                _ => 0x00,
            })
            .collect();

        PkmnapiDBString { value }
    }

    /// Decodes the ROM string
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    ///
    /// let string = PkmnapiDBString::new(&[0x80, 0x81, 0x82, 0x50]);
    /// let decoded = string.decode();
    ///
    /// assert_eq!(decoded, "ABC@");
    /// ```
    pub fn decode(&self) -> String {
        let value: String = self
            .value
            .iter()
            .map(|c| match c {
                0x50 => "@",
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
                0xE0 => "'",
                0xE3 => "-",
                0xE8 => ".",
                0xEF => "♂",
                0xF5 => "♀",
                _ => "?",
            })
            .collect();

        value
    }

    /// Decodes the ROM string and trims after the terminator character (@)
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    ///
    /// let string = PkmnapiDBString::new(&[0x80, 0x81, 0x82, 0x50]);
    /// let decoded = string.decode_trimmed();
    ///
    /// assert_eq!(decoded, "ABC");
    /// ```
    pub fn decode_trimmed(&self) -> String {
        let value = self.decode();
        let at_offset = value.find('@').unwrap_or(value.len());
        let mut value = String::from(&value);

        value.truncate(at_offset);

        value
    }
}

impl fmt::Display for PkmnapiDBString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.decode_trimmed())
    }
}

#[cfg(test)]
mod tests {
    use crate::db::string::*;

    #[test]
    fn string_iterop() {
        let db_string = PkmnapiDBString::new(&[0x80, 0x81, 0x82, 0x50]);

        assert_eq!(db_string.value, vec![0x80, 0x81, 0x82, 0x50]);

        let string = db_string.decode();

        assert_eq!(string, "ABC@");

        let db_string_new = PkmnapiDBString::from_string(string);

        assert_eq!(db_string.value, db_string_new.value);
    }
}
