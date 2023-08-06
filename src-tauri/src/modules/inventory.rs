use proc_mem::Process;

pub fn check_inventory_full(process: &Process, character_base_ptr: usize) -> bool {
    let max_items = process.read_mem_chain::<i32>(vec![character_base_ptr, 0x278, 0x78, 0x18]).unwrap();
    
    for i in 0..max_items {
        let item_id = process.read_mem_chain::<i32>(vec![character_base_ptr, 0x278, 0x78, 0x10, 0x20 + (0x8 * i as usize), 0x18]).unwrap();

        if item_id == 0 {
            return false;
        }
    }
    
    true
}