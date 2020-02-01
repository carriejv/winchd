mod x11_window_info;

use std::thread;
use std::time::Duration;

const PROCESS_POLLING_INTERVAL_MS: u64 = 1000;

pub struct FocusedWindowInfo {
    class: Vec<String>,
    name: Vec<String>,
}

fn main() {
    loop {
        thread::sleep(Duration::from_millis(PROCESS_POLLING_INTERVAL_MS));
        let xwinfo = x11_window_info::get_focused_window_info().unwrap();
        println!("class: {:?}, name: {:?}", xwinfo.class, xwinfo.name);
        //notifica::notify("Current Window", &format!("class: {:?}, name: {:?}", xwinfo.class, xwinfo.name));
    }
    
}