
use active_win_pos_rs::{ActiveWindow, get_active_window};

fn main() {
    getcurrentprocess();


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

}