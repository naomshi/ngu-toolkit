use proc_mem::{Process, Module};

pub struct ProcessState {
    pub process: Process,
    pub module: Module,
    pub character_ptr: usize
}

impl ProcessState {
    pub fn get() -> ProcessState {
        let ngu_idle = Process::with_name("NGUIdle.exe")
            .expect("ERROR: Could not find NGU Idle process. ...");
        let unity_player = ngu_idle.module("UnityPlayer.dll")
            .expect("Could not locate UnityPlayer.dll module. ...");
        let character_ptr = ngu_idle
            .read_mem_chain::<usize>(vec![unity_player.base_address(), 0x017A52A8, 0x100, 0x18])
            .unwrap();

        ProcessState{
            process: ngu_idle,
            module: unity_player,
            character_ptr: character_ptr
        }
    }
}