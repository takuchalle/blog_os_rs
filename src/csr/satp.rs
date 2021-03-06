// Supervisor Address Translation and Protection (satp) register
//

pub struct Satp {
    pub mode: Mode,
}

#[derive(Copy, Clone)]
pub enum Mode {
    Bare = 0,
    Sv39 = 8,
}

impl Satp {
    pub fn read() -> Self {
        let bits = read_csr!(0x180);
        let mode = match bits >> 30 {
            0 => Mode::Bare,
            8 => Mode::Sv39,
            _ => unimplemented!(),
        };
        Self { mode }
    }

    pub fn write(&self) {
        let bits = (self.mode as usize) << 30;
        write_csr!(0x180, bits);
    }
}
