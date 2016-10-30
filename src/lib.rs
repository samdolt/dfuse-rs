// Copyright © 2016 - Samuel Dolt <samuel@dolt.ch>
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # DfuSe library for Rust
//!
//! This library can read and write DfuSe file `file.dfu`
//!
//! ## What is DfuSe
//!
//! `DfuSe` is an acronym for `DFU STMicroelectronics Extension`
//!
//! ## What is DFU
//!
//! `DFU` is an acronym for `Device Firmware Upgrade`. It's a standard USB protocol
//! to upgrade device.
//!
//! # Resources
//!
//! Useful ressource about DfuSe:
//!
//! - DfuSe File Format Specification Rev 1: [link](http://rc.fdr.hu/UM0391.pdf)
//! - USB Device Class Specification for DFU V 1.1: [link](http://www.usb.org/developers/docs/devclass_docs/DFU_1.1.pdf)

// #![deny(missing_docs)]
// #![deny(warnings)]

extern crate byteorder;
mod file;
pub use file::DfuseFile;

pub mod elements;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
