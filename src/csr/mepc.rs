#[derive(Copy, Clone, Debug)]
pub struct Mepc {}

impl Mepc {
    pub fn write(pc: usize) {
        write_csr!(0x341, pc);
    }
}
