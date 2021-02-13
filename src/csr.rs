macro_rules! write_csr {
    ($register:expr, $value:expr) => {
        unsafe { asm!("csrrw x0, {register}, {}",
                      in(reg) $value,
                      register = const $register)
        }
    };
}

macro_rules! read_csr{
    ($register:expr) => {{
        let result: usize;
        unsafe {
            asm!("csrr {}, {register}",
                 out(reg) result,
                 register = const $register);
        }
        result
    }}
}

pub mod medeleg;
pub mod mepc;
pub mod mstatus;
