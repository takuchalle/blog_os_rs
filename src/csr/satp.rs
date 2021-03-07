// Supervisor Address Translation and Protection (satp) register
//

use bit_field::BitField;

pub struct Satp {
    pub mode: Mode,
    pub asid: usize,
    pub ppw: usize,
}

#[derive(Copy, Clone)]
pub enum Mode {
    Bare = 0,
    Sv39 = 8,
}

impl Satp {
    pub fn read() -> Self {
        let bits = read_csr!(0x180);
        let mode = match bits.get_bits(60..=63) {
            0 => Mode::Bare,
            8 => Mode::Sv39,
            _ => unimplemented!(),
        };
        let asid = bits.get_bits(44..=59);
        let ppw = bits.get_bits(0..=43);
        Self { mode, asid, ppw }
    }

    pub fn write(&self) {
        let mut bits = 0;
        bits.set_bits(60..=63, self.mode as usize);
        bits.set_bits(44..=59, self.asid);
        bits.set_bits(0..=43, self.ppw);

        write_csr!(0x180, bits);
    }
}
