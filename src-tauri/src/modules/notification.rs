use std::{ffi::OsString, os::windows::prelude::OsStringExt, collections::HashMap, sync::{Arc, Mutex}, thread, time::Duration};
use serde::Deserialize;
use winapi::{um::winuser::{GetForegroundWindow, GetWindowTextW}, shared::minwindef::MAX_PATH};
use winrt_notification::{Sound, Toast, Duration as NotifDuration};

use crate::modules::adventure;

use super::{questing, util::ProcessState};

#[derive(Eq, Hash, PartialEq, Deserialize, Debug, Clone)]
pub enum TimerName {
    Quest,
    Adventure,
    Cooking,
}

pub struct TimerStatus {
    pub timers: HashMap<TimerName, bool>,
}

impl TimerStatus {
    pub fn new() -> Self {
        let mut timers = HashMap::new();
        timers.insert(TimerName::Quest, false);
        timers.insert(TimerName::Adventure, false);
        timers.insert(TimerName::Cooking, false);

        TimerStatus {
            timers
        }
    }
}

fn get_active_window_title() -> Option<String> {
    unsafe {
        let hwnd = GetForegroundWindow();
        let mut title_os: Vec<u16> = vec![0; MAX_PATH];
        let len = GetWindowTextW(hwnd, title_os.as_mut_ptr(), title_os.len() as i32);
        if len > 0 {
            let title_os = OsString::from_wide(&title_os[..len as usize]);
            title_os.into_string().ok()
        } else {
            None
        }
    }
}

fn show_notification(title: &str, body: &str) -> () {
    Toast::new("steam://rungameid/1147690")
        .duration(NotifDuration::Long)
        .title(title)
        .text1(body)
        .sound(Some(Sound::SMS)) // will be silent
        .show()
        .expect("unable to toast");

    // Wait until NGU Idle is focused
    while get_active_window_title().unwrap_or_default() != "NGU Idle" {
        thread::sleep(Duration::from_millis(500));
    }

    // Wait until NGU Idle is unfocused
    while get_active_window_title().unwrap_or_default() == "NGU Idle" {
        thread::sleep(Duration::from_millis(500));
    }
}

pub fn check_and_notify(timer_status: Arc<Mutex<TimerStatus>>) {
    let process_state = ProcessState::get();

    loop {
        // Create a clone of the timers here to avoid blocking
        let timers = {
            let status = timer_status.lock().unwrap();
            status.timers.clone()
        };
        
        if *timers.get(&TimerName::Quest).unwrap_or(&false) {
            if questing::quest_check(&process_state.process, &process_state.module, process_state.character_ptr){
                show_notification("Quest", "You have enough items to complete your quest.");
            }
        }
    
        if *timers.get(&TimerName::Adventure).unwrap_or(&false) {
            if adventure::check_idle(&process_state.process, &process_state.module) {
                show_notification("Adventure", "You are idle in adventure mode.");
            }
        }
    
        // Make sure to sleep for a while to prevent the loop from running continuously
        std::thread::sleep(std::time::Duration::from_secs(1));
    }    
}