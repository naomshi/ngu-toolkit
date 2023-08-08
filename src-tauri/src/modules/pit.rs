use serde::Serialize;

use crate::modules::util::XORShift128;

use super::util::ProcessState;

#[derive(Serialize, Debug, Clone, Copy)]
pub enum PitReward {
    EquipmentShockwave,
    DaycareShockwave,
    IronPill,
    Exp,
    Seeds,
    InfPower,
    InfToughness,
    InfBoth,
    Power,
    Toughness,
    MaxHp,
    HpRegen,
    Wandoos
}

impl PitReward {
    pub fn description(&self) -> &'static str {
        match self {
            PitReward::EquipmentShockwave => "Equipment Shockwave",
            PitReward::DaycareShockwave => "Daycare Shockwave",
            PitReward::IronPill => "Iron Pill",
            PitReward::Exp => "Experience",
            PitReward::Seeds => "Yggdrasil Seeds",
            PitReward::InfPower => "Infinity Cube Power",
            PitReward::InfToughness => "Infinity Cube Toughness",
            PitReward::InfBoth => "Infinity Cube Power and Toughness",
            PitReward::Power => "Character Power",
            PitReward::Toughness => "Character Toughness",
            PitReward::MaxHp => "Character Max HP",
            PitReward::HpRegen => "Character HP Regen",
            PitReward::Wandoos => "Wandoos Level"
        }
    }

    pub fn img(&self) -> &'static str {
        match self {
            PitReward::EquipmentShockwave => "equipment_shockwave.png",
            PitReward::DaycareShockwave => "daycare_shockwave.png",
            PitReward::IronPill => "iron_pill.png",
            PitReward::Exp => "exp.png",
            PitReward::Seeds => "seeds.png",
            PitReward::InfPower => "inf_power.png",
            PitReward::InfToughness => "inf_toughness.png",
            PitReward::InfBoth => "inf_both.png",
            PitReward::Power => "power.png",
            PitReward::Toughness => "toughness.png",
            PitReward::MaxHp => "max_hp.png",
            PitReward::HpRegen => "hp_regen.png",
            PitReward::Wandoos => "wandoos.png"
        }
    }
}

#[derive(Serialize, Debug)]
pub struct PitRewardMapping {
    pub index: PitReward,
    pub description: &'static str,
    pub img: &'static str
}

#[tauri::command]
pub fn get_pit_rewards() -> Vec<PitRewardMapping> {
    let process_state = ProcessState::get();

    let pit_wish = process_state.process.read_mem_chain::<i32>(vec![process_state.character_ptr, 0x350, 0x10, 0x10, 0x40, 0x28]).unwrap();

    let gold = process_state.process.read_mem_chain::<f64>(vec![process_state.character_ptr, 0x3c0]).unwrap();

    let gold_log10 = gold.log10();

    let mut state = process_state.process.read_mem_chain::<XORShift128>(vec![process_state.character_ptr, 0x2e8, 0x28]).unwrap();

    if (gold_log10 >= 50.0) && (pit_wish == 1) {
        let rewards = (0..5).map(|i| {
            let reward_index = state.next_int_range(1, 6);

            let pit_reward = match reward_index {
                1 => PitReward::IronPill,
                2 => PitReward::EquipmentShockwave,
                3 => PitReward::Exp,
                4 => PitReward::Seeds,
                5 => PitReward::DaycareShockwave,
                _ => panic!("Invalid pit index")
            };

            PitRewardMapping{
                index: pit_reward,
                description: pit_reward.description(),
                img: pit_reward.img()
            }

        }).collect();

        return rewards;
    }

    let rewards = (0..5).map(|i| {
        let reward_index = state.next_int_range(1, 13);
        
        let pit_reward = match reward_index {
            1 => PitReward::InfPower,
            2 => PitReward::InfToughness,
            3 => PitReward::InfBoth,
            4 => PitReward::EquipmentShockwave,
            5 => PitReward::Exp,
            6 => PitReward::Seeds,
            7 => PitReward::Power,
            8 => PitReward::Toughness,
            9 => PitReward::MaxHp,
            10 => PitReward::HpRegen,
            11 => PitReward::Wandoos,
            12 => PitReward::DaycareShockwave,
            _ => panic!("Invalid pit index")
        };

        PitRewardMapping{
            index: pit_reward,
            description: pit_reward.description(),
            img: pit_reward.img()
        }        
    }).collect();

    return rewards;
}