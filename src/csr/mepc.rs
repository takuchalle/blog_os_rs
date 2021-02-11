#[derive(Copy, Clone, Debug)]
pub struct Mepc {}

impl Mepc {
    pub fn write(pc: usize) {
        unsafe { asm!("csrrw x0, mepc, {}", in(reg) pc) }
    }
}
