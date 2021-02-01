use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref UART: Mutex<Uart> = Mutex::new(Uart {
        addr: unsafe { &mut *(0x1000_0000 as *mut Volatile<u8>) }
    });
}

pub struct Uart {
    addr: &'static mut Volatile<u8>,
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    UART.lock().write_fmt(args).unwrap();
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.addr.write(c as u8);
        }
        Ok(())
    }
}
