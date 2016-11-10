// Copyright © 2016 - Samuel Dolt <samuel@dolt.ch>
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those term

// Based on: http://www.sunshine2k.de/articles/coding/crc/understanding_crc.html
// List of common crc: http://www.sunshine2k.de/articles/coding/crc/understanding_crc.html

fn swap8(in_v: u8) -> u8 {
    let mut out_v = 0u8;
    for pos in 0..8 {
        let bit = (in_v & (1 << pos)) >> pos;
        out_v |= bit << (7 - pos);
    }
    out_v
}

fn swap32(in_v: u32) -> u32 {
    let mut out_v = 0u32;
    for pos in 0..32 {
        let bit = (in_v & (1 << pos)) >> pos;
        out_v |= bit << (31 - pos);
    }
    out_v
}


pub struct CRC32 {
    poly: u32,
    ref_in: bool,
    ref_out: bool,
    initial: u32,
    xor_out: u32,

    value: u32,
    table: [u32; 256],
}

impl CRC32 {
    pub fn new_jam() -> CRC32 {
        let crc = CRC32 {
            poly: 0x04C11DB7,
            ref_in: true,
            ref_out: true,
            initial: 0xFFFFFFFF,
            xor_out: 0,
            value: 0,
            table: [0u32; 256],
        };

        crc._new()
    }

    pub fn _new(mut self) -> CRC32 {
        self._init_table();
        self.reset();
        self
    }
    #[inline]
    fn _init_table(&mut self) {
        for i in 0u32..256u32 {
            let mut cur: u32 = i << 24;

            for _ in 0..8 {
                if (cur & 0x80000000) != 0 {
                    cur <<= 1;
                    cur ^= self.poly;
                } else {
                    cur <<= 1;
                }
            }

            self.table[i as usize] = cur;
        }

        for b in self.table.iter() {
            println!("{:X}", b)
        }

    }

    pub fn reset(&mut self) {
        self.value = self.initial;
    }

    #[inline]
    pub fn add(&mut self, b: u8) {
        let b: u8 = match self.ref_in {
            true => swap8(b),
            false => b,
        };

        let pos = ((self.value ^ ((b as u32) << 24)) >> 24) as u8;

        self.value = (self.value << 8) ^ (self.table[pos as usize]);
    }

    #[inline]
    fn _finalize(&mut self) -> u32 {
        let value = match self.ref_out {
            true => swap32(self.value),
            false => self.value,
        };

        value ^ self.xor_out
    }

    pub fn finalize(mut self) -> u32 {
        self._finalize()
    }

    pub fn get_and_reset(&mut self) -> u32 {
        let v = self._finalize();
        self.reset();
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static CRC_DEFAULT_CHECK: &'static str = "123456789";

    fn crc_check(mut crc: CRC32) -> u32 {
        for b in CRC_DEFAULT_CHECK.as_bytes() {
            crc.add(*b);
        }
        crc.finalize()
    }
    #[test]
    fn test_crc_jam_give_correct_value() {
        let crc = CRC32::new_jam();
        assert_eq!(crc_check(crc), 0x340BC6D9);
    }

}
