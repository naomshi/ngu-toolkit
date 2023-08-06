use proc_mem::{Module, Process};

#[repr(C)]
#[derive(Debug, Default)]
struct InventoryList {
    _meta: u128,
    _items: usize, // Pointer to raw items array
    _size: i32   // Amount of items in array
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct Equipment {
    _meta: u128,
    path: usize,
    id: i32,
    e_type: i32, // todo: implement part enum
    boss_required: i32,
    cap_attack: f32,
    cur_attack: f32,
    cap_defense: f32,
    cur_defense: f32,
    spec_1_type: i32, // todo: implement specType enum
    spec_1_cur: f32,
    spec_1_cap: f32,
    spec_2_type: i32, // todo: implement specType enum
    spec_2_cur: f32,
    spec_2_cap: f32,
    spec_3_type: i32, // todo: implement specType enum
    spec_3_cur: f32,
    spec_3_cap: f32,
    removable: bool,
    num_spec: i32,
    level: i32,
    unique: bool
}

pub fn get_inventory_items(process: &Process, module: &Module) -> Vec<Equipment> {
    let chain = vec![module.base_address(), 0x017a52a8, 0x100, 0x18, 0x278, 0x78, 0x0];

    let list = process.read_mem_chain::<InventoryList>(chain).unwrap();

    let mut items: Vec<Equipment> = vec![];

    for i in 0..list._size.try_into().unwrap() { 
        let item = process.read_mem_chain::<Equipment>(vec![list._items, (i * 8) + 0x20, 0x0]).unwrap();

        items.push(item);
    }

    return items;
}

pub fn check_quest(process: &Process, module: &Module, character_ptr: usize) -> bool {
    let beast_quest_ptr = process.read_mem_chain::<usize>(vec![character_ptr, 0x338]).unwrap();

    let quest_zone = process.read_mem_chain::<i32>(vec![beast_quest_ptr, 0x34]).unwrap();

    let quest_target = process.read_mem_chain::<i32>(vec![beast_quest_ptr, 0x38]).unwrap();

    let quest_progress = process.read_mem_chain::<i32>(vec![beast_quest_ptr, 0x3c]).unwrap();

    let item_counter = get_inventory_items(process, module).iter().filter(|item| {
        item.id == quest_zone
    })
    .count() as i32;

    // add together and return true/false if over limit
    let progress = item_counter + quest_progress;

    println!("Progress: {}/{} ({} handed in, {} in inventory)", progress, quest_target, quest_progress, item_counter);

    progress >= quest_target
}