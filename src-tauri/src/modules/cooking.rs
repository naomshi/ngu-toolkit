use std::mem::size_of;

use proc_mem::Process;
use serde::Serialize;

use super::util::ProcessState;

#[derive(Debug)]
struct Cooking {
    ingredients: Vec<Ingredient>,
    pair_1: Pair,
    pair_2: Pair,
    pair_3: Pair,
    pair_4: Pair,
    pair_1_target: i32,
    pair_2_target: i32,
    pair_3_target: i32,
    pair_4_target: i32,
}

#[derive(Default, Debug)]
struct Pair {
    ingredient_1: usize,
    ingredient_2: usize,
}

#[derive(Default, Debug)]
struct Ingredient {
    property_index: i32,
    _cur_level: i32,
    target_level: i32,
    _paired_ingredient: i32,
    weight: f32,
    paired_weight: f32,
    unlocked: bool,
}

#[derive(Serialize)]
pub struct SolvedIngredient {
    pub property_index: i32,
    pub optimal_value: i32
}

impl Pair {
    fn from_process(process: &Process, cooking_base_ptr: usize, pair_no: usize) -> Pair {
        let val_1 = process.read_mem_chain::<i32>(vec![cooking_base_ptr, 0x18 + (0x8 * pair_no), 0x10, 0x20]).unwrap();
        let val_2 = process.read_mem_chain::<i32>(vec![cooking_base_ptr, 0x18 + (0x8 * pair_no), 0x10, 0x24]).unwrap();
        
        Pair {
            ingredient_1: val_1 as usize,
            ingredient_2: val_2 as usize
        }
    }
}

impl Cooking {
    fn from_process(process: &Process, character_base_ptr: usize) -> Cooking {
        let cooking_base_ptr = process.read_mem::<usize>(character_base_ptr + 0x368).unwrap();

        let items_ptr = process.read_mem_chain::<usize>(vec![cooking_base_ptr, 0x10, 0x10]).unwrap();

        let ingredients: Vec<Ingredient> = (0..8).map(|i|{
            let item_ptr = items_ptr + 0x20 + (size_of::<usize>() * i);
            let item_obj = process.read_mem::<usize>(item_ptr).unwrap(); 
            process.read_mem::<Ingredient>(item_obj + 0x10).unwrap()
        }).collect();

        Cooking {
            ingredients: ingredients,
            pair_1: Pair::from_process(process, cooking_base_ptr, 0),
            pair_2: Pair::from_process(process, cooking_base_ptr, 1),
            pair_3: Pair::from_process(process, cooking_base_ptr, 2),
            pair_4: Pair::from_process(process, cooking_base_ptr, 3),
            pair_1_target: process.read_mem::<i32>(cooking_base_ptr + 0x40).unwrap(),
            pair_2_target: process.read_mem::<i32>(cooking_base_ptr + 0x44).unwrap(),
            pair_3_target: process.read_mem::<i32>(cooking_base_ptr + 0x48).unwrap(),
            pair_4_target: process.read_mem::<i32>(cooking_base_ptr + 0x4c).unwrap(),
        }
    }

    fn get_local_score(&self, ingredient_index: usize, ingredient_level: i32) -> f64 {
        let ingredient = &self.ingredients[ingredient_index];
        (1.0 - 0.03 * (ingredient.target_level as f64 - ingredient_level as f64).abs()).powf(30.0)
            * ingredient.weight as f64
    }

    fn get_paired_score(&self, pair_index: usize, total_ingredient_level: i32) -> f64 {
        let pair = match pair_index {
            1 => &self.pair_1,
            2 => &self.pair_2,
            3 => &self.pair_3,
            4 => &self.pair_4,
            _ => panic!("Invalid pair index"),
        };
        let target_level = match pair_index {
            1 => self.pair_1_target,
            2 => self.pair_2_target,
            3 => self.pair_3_target,
            4 => self.pair_4_target,
            _ => panic!("Invalid pair index"),
        };
        (1.0 - 0.02 * (target_level as f64 - total_ingredient_level as f64).abs()).powf(40.0)
            * self.ingredients[pair.ingredient_1].paired_weight as f64
    }

    fn max_ingredient_level(&self) -> i32 {
        20
    }
}

fn get_optimal_score(cooking: &Cooking) -> (f64, Vec<SolvedIngredient>) {
    let mut optimal_score = 0.0;
    let mut optimal_levels: Vec<(usize, usize)> = vec![(0, 0); 4]; // there are 4 pairs

    for pair_index in 1..=4 {
        let mut optimal_pair_score = 0.0;
        let mut optimal_pair_levels = (0, 0);

        let pair = match pair_index {
            1 => &cooking.pair_1,
            2 => &cooking.pair_2,
            3 => &cooking.pair_3,
            4 => &cooking.pair_4,
            _ => panic!("Invalid pair index"),
        };

        let ingred_1_unlocked = cooking.ingredients[pair.ingredient_1].unlocked;
        let ingred_2_unlocked = cooking.ingredients[pair.ingredient_2].unlocked;
        if !(ingred_1_unlocked && ingred_2_unlocked) {
            // If one of the ingredients in the pair is locked
            // Use the target level of the unlocked ingredient as optimal
            if ingred_1_unlocked {
                optimal_pair_levels.0 =
                    cooking.ingredients[pair.ingredient_1].target_level as usize;
            }
            if ingred_2_unlocked {
                optimal_pair_levels.1 =
                    cooking.ingredients[pair.ingredient_2].target_level as usize;
            }
            optimal_score +=
                cooking.get_local_score(pair.ingredient_1, optimal_pair_levels.0 as i32);
            optimal_score +=
                cooking.get_local_score(pair.ingredient_2, optimal_pair_levels.1 as i32);
        } else {
            // Both ingredients in the pair are unlocked
            // Find the optimal levels
            for ingred_level_1 in 0..=cooking.max_ingredient_level() {
                for ingred_level_2 in 0..=cooking.max_ingredient_level() {
                    let mut pair_score = 0.0;
                    if cooking.ingredients[pair.ingredient_1].unlocked {
                        pair_score += cooking.get_local_score(pair.ingredient_1, ingred_level_1);
                        pair_score += cooking.get_local_score(pair.ingredient_1, ingred_level_2);
                    }
                    if cooking.ingredients[pair.ingredient_2].unlocked {
                        pair_score += cooking.get_local_score(pair.ingredient_2, ingred_level_1);
                        pair_score += cooking.get_local_score(pair.ingredient_2, ingred_level_2);
                    }
                    if cooking.ingredients[pair.ingredient_1].unlocked
                        && cooking.ingredients[pair.ingredient_2].unlocked
                    {
                        pair_score +=
                            cooking.get_paired_score(pair_index, ingred_level_1 + ingred_level_2);
                    }

                    if pair_score > optimal_pair_score {
                        optimal_pair_score = pair_score;
                        optimal_pair_levels = (ingred_level_1 as usize, ingred_level_2 as usize);
                    }
                }
            }
            optimal_score += optimal_pair_score;
        }

        optimal_levels[pair_index - 1] = optimal_pair_levels;
    }

    // Prepare a vector for SolvedIngredients
    let mut ingredients: Vec<i32> = vec![0; cooking.ingredients.len()];

    for (i, levels) in optimal_levels.iter().enumerate() {
        let pair = match i {
            0 => &cooking.pair_1,
            1 => &cooking.pair_2,
            2 => &cooking.pair_3,
            3 => &cooking.pair_4,
            _ => panic!("Invalid pair index"),
        };
        if cooking.ingredients[pair.ingredient_1].unlocked {
            ingredients[pair.ingredient_1] = levels.0 as i32;
        }
        if cooking.ingredients[pair.ingredient_2].unlocked {
            ingredients[pair.ingredient_2] = levels.1 as i32;
        }
    }

    let solved: Vec<SolvedIngredient> = ingredients
        .into_iter()
        .enumerate()
        .map(|(i, optimal_value)| SolvedIngredient {
            property_index: cooking.ingredients[i].property_index,
            optimal_value,
        })
        .collect();

    (optimal_score, solved)
}

#[tauri::command]
pub fn solve_cooking() -> Result<Vec<SolvedIngredient>, String> {
    let process_state = ProcessState::get();

    let cooking = Cooking::from_process(&process_state.process, process_state.character_ptr);

    let (_, solved) = get_optimal_score(&cooking);

    let result: Vec<SolvedIngredient> = solved.into_iter()
        .filter(|ing| cooking.ingredients.iter().any(|item| item.property_index == ing.property_index && item.unlocked))
        .collect();

    Ok(result)
}
