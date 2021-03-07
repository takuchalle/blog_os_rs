#![no_std]
#![no_main]
#![feature(asm)]

use blog_os_rs::csr;
use blog_os_rs::println;

use core::panic::PanicInfo;

extern "C" {
    static trap_entry: u8;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _main() -> ! {
    println!("main");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut mstatus = csr::mstatus::Mstatus::read();
    mstatus.set_mpp(csr::mstatus::Mpp::Super);
    mstatus.write();

    let addr = unsafe { &trap_entry as *const u8 } as usize;
    println!("addr {:X}", addr);
    csr::mepc::Mepc::write(addr);

    // deleget all interrupt/exception to supervisor mode
    csr::medeleg::Medeleg::delegate_all();
    csr::mideleg::Mideleg::delegate_all();

    let mut sie = csr::sie::Sie::read();
    sie.enable_seie();
    sie.enable_stie();
    sie.enable_ssie();
    sie.write();

    let satp = csr::satp::Satp{ mode: csr::satp::Mode::Bare, asid: 0, ppw: 0 };
    satp.write();

    // switch to supervisor mode
    unsafe { asm!("mret") }

    loop {}
}
