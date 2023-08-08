// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;

use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use modules::{notification::{TimerStatus, TimerName, self}, cooking, pit};

static TIMER_STATUS: Lazy<Arc<Mutex<TimerStatus>>> = Lazy::new(|| Arc::new(Mutex::new(TimerStatus::new())));

fn get_timer_status() -> Arc<Mutex<TimerStatus>> {
    Arc::clone(&TIMER_STATUS)
}

#[tauri::command]
fn enable_timer(timer_name: TimerName) {
    let timer_status = get_timer_status();
    let mut status = timer_status.lock().unwrap();

    println!("Enabling timer {:?}", timer_name);

    status.timers.insert(timer_name, true);
}

#[tauri::command]
fn disable_timer(timer_name: TimerName) {
    let timer_status = get_timer_status();
    let mut status = timer_status.lock().unwrap();

    println!("Disabling timer {:?}", timer_name);

    status.timers.insert(timer_name, false);
}

#[tauri::command]
fn get_timer(timer_name: TimerName) -> bool {
    let timer_status = get_timer_status();
    let status = timer_status.lock().unwrap();
    *status.timers.get(&timer_name).unwrap_or(&false)
}

fn main() {
    std::thread::spawn(move || {
        notification::check_and_notify(get_timer_status());
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            enable_timer,
            disable_timer,
            get_timer,
            cooking::solve_cooking,
            pit::get_pit_rewards
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
