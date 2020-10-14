pub struct Bitstream<'a> {
    pub value: Box<dyn Iterator<Item = u8> + 'a>,
    pub bits: usize,
}

impl<'a> Bitstream<'a> {
    pub fn from(data: &'a [u8]) -> Self {
        let value = data
            .iter()
            .map(|byte| (0..=7).map(move |i| (*byte & (0x01 << (7 - i))) >> (7 - i)))
            .flatten();

        Bitstream {
            value: Box::new(value),
            bits: 0,
        }
    }

    pub fn get(&mut self, length: u32) -> u32 {
        let output = self
            .value
            .by_ref()
            .take(length as usize)
            .enumerate()
            .map(|(i, bit)| (bit as u32) << (length - (i as u32) - 1))
            .fold(0, |acc, val| acc | val);

        self.bits += length as usize;

        output
    }

    pub fn get_until_zero(&mut self) -> (u32, u32) {
        let bits: Vec<u8> = self.value.by_ref().take_while(|bit| *bit != 0x00).collect();
        let length = (bits.len() as u32) + 1;

        let value = bits
            .iter()
            .enumerate()
            .map(|(i, bit)| (*bit as u32) << (length - (i as u32) - 1))
            .fold(0, |acc, val| acc | val);

        self.bits += length as usize;

        (value, length)
    }
}
