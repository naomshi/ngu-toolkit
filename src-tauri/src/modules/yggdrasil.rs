use proc_mem::Process;

#[repr(C)]
#[derive(Debug, Default)]
struct Fruit {
    target_time: f32,
    seconds: f32,
    activated: bool,
    unlock_cost: i64,
    total_levels: i64,
    max_tier: i64,
    perm_cost_paid: bool,
    use_poop: bool,
    eat_fruit: bool,
    harvests: i32
}

pub fn check_fruit_ready(process: &Process, character_base_ptr: usize) -> bool {
    let total_fruits = 21;
    
    for i in 0..total_fruits {
        let fruit = process.read_mem_chain::<Fruit>(vec![character_base_ptr, 0x300, 0x40, 0x10, 0x20 + (0x8 * i as usize), 0x10]).unwrap();

        if fruit.activated && (fruit.seconds >= (fruit.max_tier as f32 * 3600.0)) {
            return true;
        }
    }
    
    false
}