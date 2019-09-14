use super::ROM_DRIVER_TABLE_POINTER;

const DRIVER_OFFSET: usize = 0x24;

#[repr(C)]
pub struct Config {
    sys_clk_in_hz: u32,
    baudrate_in_hz: u32,
    config: u8,
    sync_mod: u8,
    error_en: u16,
}

#[repr(C)]
pub struct Param {
    buffer: *mut u8,
    size: u32,
    transfer_mode: u16,
    driver_mode: u16,
    callback_func_pt: extern "C" fn(),
}

#[repr(C)]
pub struct Apis {
    get_mem_size: extern "C" fn() -> usize,
    setup: extern "C" fn(base_address: u32, ram: *mut u8) -> *mut usize,
    init: extern "C" fn(handle: *mut usize, set: Config) -> u32,
    get_char: extern "C" fn(handle: *mut usize) -> u8,
    put_char: extern "C" fn(handle: *mut usize, data: u8),
    get_line: extern "C" fn(handle: *mut usize, param: Param) -> u32,
    put_line: extern "C" fn(handle: *mut usize, param: Param) -> u32,
    uart_isr: extern "C" fn(handle: *mut usize),
}

pub fn take() -> *const Apis {
    let res: *const Apis;
    unsafe {
        res = *((*ROM_DRIVER_TABLE_POINTER + DRIVER_OFFSET) as *const usize) as *const Apis;
    }
    return res;
}