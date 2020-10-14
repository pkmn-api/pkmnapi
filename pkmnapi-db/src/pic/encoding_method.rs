#[derive(Clone, Debug, PartialEq)]
pub enum EncodingMethod {
    ONE(u8),
    TWO(u8),
    THREE(u8),
}

impl EncodingMethod {
    pub fn value(&self) -> u8 {
        match self {
            EncodingMethod::ONE(primary_buffer) => *primary_buffer,
            EncodingMethod::TWO(primary_buffer) => *primary_buffer,
            EncodingMethod::THREE(primary_buffer) => *primary_buffer,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            EncodingMethod::ONE(_) => vec![0x00],
            EncodingMethod::TWO(_) => vec![0x01, 0x00],
            EncodingMethod::THREE(_) => vec![0x01, 0x01],
        }
    }

    pub fn from(encoding_method: u8, primary_buffer: u8) -> Self {
        match encoding_method {
            0x02 => EncodingMethod::TWO(primary_buffer),
            0x03 => EncodingMethod::THREE(primary_buffer),
            _ => EncodingMethod::ONE(primary_buffer),
        }
    }
}
