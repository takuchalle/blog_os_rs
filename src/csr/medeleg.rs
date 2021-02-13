pub struct Medeleg {
    bits: usize,
}

impl Medeleg {
    pub fn write(&self) {
        write_csr!(0x302, self.bits);
    }

    pub fn read() -> Self{
        let bits = read_csr!(0x302);
        Self { bits }
    }
}
