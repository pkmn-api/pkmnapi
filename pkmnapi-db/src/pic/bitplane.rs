use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

#[derive(Clone, Debug, PartialEq)]
pub struct Bitplane {
    pub data: Vec<u8>,
}

impl Bitplane {
    pub fn delta_decode(&mut self, row_size: usize) -> Bitplane {
        let mut state = 0x00;

        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(i, bit)| {
                if (i % row_size) == 0x00 {
                    state = 0x00;
                }

                if *bit == 0x00 {
                    state
                } else {
                    state ^= 0x01;

                    state
                }
            })
            .collect();

        Bitplane { data }
    }

    pub fn delta_encode(&mut self, row_size: usize) -> Bitplane {
        let mut state = 0x00;

        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(i, bit)| {
                if (i % row_size) == 0x00 {
                    state = 0x00;
                }

                if *bit == state {
                    0x00
                } else {
                    state ^= 0x01;

                    0x01
                }
            })
            .collect();

        Bitplane { data }
    }
}

impl From<Vec<u8>> for Bitplane {
    fn from(data: Vec<u8>) -> Self {
        Bitplane { data }
    }
}

impl BitOr for Bitplane {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a | b)
            .collect();

        Bitplane { data }
    }
}

impl BitAnd<u8> for Bitplane {
    type Output = Self;

    fn bitand(self, other: u8) -> Self::Output {
        let data = self.data.iter().map(|a| a & other).collect();

        Bitplane { data }
    }
}

impl BitXor for Bitplane {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        Bitplane { data }
    }
}

impl Shl<u8> for Bitplane {
    type Output = Self;

    fn shl(self, other: u8) -> Self::Output {
        let data = self.data.iter().map(|a| a << other).collect();

        Bitplane { data }
    }
}

impl Shr<u8> for Bitplane {
    type Output = Self;

    fn shr(self, other: u8) -> Self::Output {
        let data = self.data.iter().map(|a| a >> other).collect();

        Bitplane { data }
    }
}
