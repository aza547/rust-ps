use std::{thread, time};
use sysinfo::System;

fn main() {
    let mut system = System::new_all();

    loop {
        // Refresh all system information
        system.refresh_all();

        // Get list of processes
        let processes = system.processes();

        // Print process information
        println!("Running Processes:");
        for (pid, process) in processes {
            println!("PID: {}, Name: {}", pid, process.name());
        }

        // Sleep for 1 second
        thread::sleep(time::Duration::from_secs(2));
    }
}