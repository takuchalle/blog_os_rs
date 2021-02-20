pub struct Mideleg {
    bits: usize,
}

impl Mideleg {
    pub fn write(&self) {
        write_csr!(0x303, self.bits);
    }

    pub fn read() -> Self {
        let bits = read_csr!(0x303);
        Self { bits }
    }

    pub fn delegate_all() {
        write_csr!(0x303, 0xffff);
    }
}
