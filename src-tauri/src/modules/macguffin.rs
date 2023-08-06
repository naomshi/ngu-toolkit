use proc_mem::Process;

pub fn check_muffin_time(process: &Process, character_base_ptr: usize) -> bool {
    let muffin_time = process.read_mem_chain::<f64>(vec![character_base_ptr, 0x310, 0x38, 0x10]).unwrap();
    let selected_menu = process.read_mem_chain::<i32>(vec![character_base_ptr, 0x384]).unwrap();
    
    (muffin_time < 1.0) && (selected_menu == 23)
}