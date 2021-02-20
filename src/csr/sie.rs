use bit_field::BitField;
pub struct Sie {
    bits: usize,
}

enum SieField {
    // USIE = 0,
    SSIE = 1,
    // UTIE = 4,
    STIE = 5,
    // UEIE = 8,
    SEIE = 9,
}

impl Sie {
    pub fn read() -> Self {
        Sie {
            bits: read_csr!(0x104),
        }
    }

    pub fn enable_seie(&mut self) {
        self.bits.set_bit(SieField::SEIE as usize, true);
    }

    pub fn enable_stie(&mut self) {
        self.bits.set_bit(SieField::STIE as usize, true);
    }

    pub fn enable_ssie(&mut self) {
        self.bits.set_bit(SieField::SSIE as usize, true);
    }

    pub fn write(&self) {
        write_csr!(0x104, self.bits);
    }
}
