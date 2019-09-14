extern crate cortex_m_rt;
use cortex_m_rt::{entry, exception, ExceptionFrame};

extern crate lpc8xx_bootapi;
use lpc8xx_bootapi::uart;

use core::panic::PanicInfo;


// Very stupid alloc functions
static mut heap_next_start_address: *mut usize = 0 as *mut usize;
fn init_stupid_alloc() {
    unsafe {
        heap_next_start_address = cortex_m_rt::heap_start() as *mut usize;
    }
}

fn stupid_alloc(size: usize) -> *mut usize {
    let res;

    unsafe {
        res = heap_next_start_address;
        // increment start address
        let temp = heap_next_start_address as usize + size;
        heap_next_start_address = temp as *mut usize;
    }
    return res;
}

#[entry]
fn main() -> !{
    let usart_heap;
    let uart_api_ptr: *const uart::Apis;

    init_stupid_alloc();
    uart_api_ptr = uart::take();

    let memsize = uart_api_ptr.get_mem_size();

    // halt
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[exception]
unsafe fn HardFault(_f: &ExceptionFrame) -> ! {
    loop {}
}
