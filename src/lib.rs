#![no_std]

pub mod uart;

const ROM_DRIVER_TABLE_POINTER: *const usize = 0x1FFF1FF8 as *const usize;
