extern crate aravis_sys;

use aravis_sys::*;

fn main() {
    println!("Hello, world!");
    unsafe { arv_update_device_list(); }
}
