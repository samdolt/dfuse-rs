extern crate dfuse;

use dfuse::DfuseFile;

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
    let ob: Vec<u8> = vec![0x00, 0xFF, 0x55, 0xAA, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF];

    // He know that option byte for an STM32F042 can be readed or writed
    // on dfuse alternate image number 1

    file.add_unamed_image(1, adress, ob);

    // He create a dfu file and compare with a reference
    let mut real_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("test_stm32f042_ob.dfu")
        .unwrap();


    file.write_to(&mut real_file).unwrap();


}