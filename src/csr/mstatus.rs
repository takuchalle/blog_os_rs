use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Mstatus {
    state: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mpp {
    User = 0,
    Super = 1,
    Machine = 3,
}

impl Mstatus {
    pub fn read() -> Mstatus {
        let result: usize;
        unsafe {
            asm!("csrr {}, mstatus", out(reg) result);
        }
        Mstatus { state: result }
    }

    pub fn write(&self) {
        unsafe { asm!("csrrw x0, mstatus, {}", in(reg) self.state) }
    }

    pub fn mie(&self) -> bool {
        self.state.get_bit(3)
    }

    pub fn set_mpp(&mut self, mpp: Mpp) {
        self.state.set_bits(11..12, mpp as usize);
    }
}
