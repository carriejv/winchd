mod window_info;

use std::thread;
use std::time::Duration;
use window_info::x11_window_info;

const PROCESS_POLLING_INTERVAL_MS: u64 = 1000;


fn main() {
    loop {
        thread::sleep(Duration::from_millis(PROCESS_POLLING_INTERVAL_MS));
        let xwinfo = x11_window_info::get_active_window_info().unwrap();
        println!("class: {:?}, name: {:?}", xwinfo.class, xwinfo.name);
        //notifica::notify("Current Window", &format!("class: {:?}, name: {:?}", xwinfo.class, xwinfo.name));
    }
    
}