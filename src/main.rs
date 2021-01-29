#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let uart0 = 0x1000_0000 as *mut u8;
    for c in b"Hello from Rust!".iter() {
        unsafe {
            *uart0 = *c as u8;
        }
    }
    loop {}
}
