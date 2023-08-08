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

#[derive(Default, Debug)]
pub struct XORShift128 {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XORShift128 {
    const MT19937: u32 = 1812433253;

    pub fn new(seed: i32) -> Self {
        let x = seed as u32;
        let y = XORShift128::MT19937.wrapping_mul(x).wrapping_add(1);
        let z = XORShift128::MT19937.wrapping_mul(y).wrapping_add(1);
        let w = XORShift128::MT19937.wrapping_mul(z).wrapping_add(1);

        XORShift128 { x, y, z, w }
    }

    pub fn with_state(x: u32, y: u32, z: u32, w: u32) -> Self {
        XORShift128 { x, y, z, w }
    }

    pub fn xorshift(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
        self.w
    }

    pub fn next_uint_range(&mut self, min: u32, max: u32) -> u32 {
        if max == min {
            return min;
        }
        
        if max < min {
            return min - self.xorshift() % (max + min);
        } else {
            return min + self.xorshift() % (max - min);
        }
    }

    pub fn next_int_range(&mut self, min: i32, max: i32) -> i32 {
        if max == min {
            return min;
        }

        let min_long = min as i64;
        let max_long = max as i64;
        let r = self.xorshift() as i64;

        if max < min {
            return (min_long - r % (max_long - min_long)) as i32;
        } else {
            return (min_long + r % (max_long - min_long)) as i32;
        }
    }
}
