// Copyright © 2016 - Samuel Dolt <samuel@dolt.ch>
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate dfuse;
extern crate byteorder;

use dfuse::DfuseFile;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};


#[test]
fn create_empty_file() {
    // Joel want to create a DFU file,
    // after a search on crates.io, it found this crate.
    //
    // He decides to give it a try, by creating a minimal dfu file

    let mut file = DfuseFile::new();

    // He has a STM32F042 based device, a minimual dfu file would
    // a file that change device's option byte

    // He found default option byte value  and start adress in RM0091 p.75 to 78

    let adress = 0x1FFFF800u32;
    let ob_word: Vec<u32> = vec![0x00FF55AA, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF];
    let mut ob: Vec<u8> = Vec::new();
    for word in ob_word {
        ob.write_u32::<LittleEndian>(word);
    }


    // He know that option byte for an STM32F042 can be readed or writed
    // on dfuse alternate image number 1. Image should be called "Option Bytes"

    file.add_image("Option Bytes  ", 1, adress, ob);
    // file.add_unamed_image(1, adress, ob);

    // He found some magic number on a forum
    // These number come from reverse engineering of a DFU file created by ST tools
    file.set_vendor_id(0x0483);
    file.set_product_id(0xDF11);
    file.set_version(0x2200);

    // He create a dfu file and compare with a reference
    let mut real_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("test_stm32f042_ob.dfu")
        .unwrap();


    file.write_to(&mut real_file).unwrap();


}