#![no_std]
#![no_main]

use blog_os_rs::println;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let x = 3;
    println!("Hello, World! {}", x);
    loop {}
}
