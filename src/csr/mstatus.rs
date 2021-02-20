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
        Mstatus {
            state: read_csr!(0x300),
        }
    }

    pub fn write(&self) {
        write_csr!(0x300, self.state);
    }

    pub fn mie(&self) -> bool {
        self.state.get_bit(3)
    }

    pub fn set_mpp(&mut self, mpp: Mpp) {
        self.state.set_bits(11..12, mpp as usize);
    }
}
