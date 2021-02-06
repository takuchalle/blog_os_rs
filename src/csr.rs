
#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Direct,
    Vectored,
    Reserved,
}

#[derive(Copy, Clone, Debug)]
pub struct MTVEC {
    pub mode: Mode,
    pub base: usize,
}

impl MTVEC {
    pub fn read() -> MTVEC{
        let result: usize;
        unsafe {
            asm!("csrr {}, mtvec", out(reg) result);
        }
        MTVEC {
            base: result & !0b11,
            mode: match result & 0b11 {
                0 => Mode::Direct,
                1 => Mode::Vectored,
                _ => Mode::Reserved,
            }
        }
    }
}
