extern crate winapi;

use std::collections::HashMap;
use std::{thread, time};

mod api;
mod structs;

fn main() {
    println!("Window Arranger is now running!");

    let mut monitor_count: u16;
    let closed = false;
    let sleep_duration = time::Duration::from_millis(10000 as u64);
    // let wait_duration = time::Duration::from_millis(5000 as u64);

    let mut monitor_map: HashMap<u16, Vec<structs::Program>> = HashMap::new();
    let mut last_count: u16 = 0;

    while !closed {
        let monitors = api::get_all_monitors();
        monitor_count = monitors.len() as u16;

        if last_count != 0 && last_count != monitor_count {
            // number of monitors changed, rearrange if applicable
            println!("Monitor count changed: {} => {}", last_count, monitor_count);

            if monitor_map.contains_key(&monitor_count) {
                let p = monitor_map.get(&monitor_count).unwrap();
                println!(
                    "There are {} programs running with this set of monitors",
                    p.len()
                );
                // thread::sleep(wait_duration);
                for program in p {
                    // iterate each program

                    if api::is_application_window(program.window) {
                        // restore placement

                        api::restore_placement(program.window, program.placement);
                    }
                }
            }
        } else {
            let programs = api::get_programs();

            println!("Updating the monitor map for {} monitors...", monitor_count);
            // update window map
            if monitor_map.contains_key(&monitor_count) {
                monitor_map.remove(&monitor_count);
            }
            monitor_map.insert(monitor_count, programs);
        }

        last_count = monitor_count;

        thread::sleep(sleep_duration);
    }
}
