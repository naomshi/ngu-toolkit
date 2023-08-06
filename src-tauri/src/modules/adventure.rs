use proc_mem::{Process, Module};

pub fn check_idle(process: &Process, module: &Module) -> bool {
    let chain = vec![module.base_address(), 0x017A52A8, 0x80, 0x200];

    let adv_zone = process.read_mem_chain::<i32>(chain).unwrap();

    adv_zone == -1
}