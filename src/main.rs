use std::{thread, time};
use sysinfo::System;

fn main() {
    let mut system = System::new_all();

    loop {
        system.refresh_all();

        let mut retail_processes = system.processes_by_exact_name("Wow.exe");
        let mut retail_beta_processes = system.processes_by_exact_name("WowB.exe");
        let mut retail_ptr_processes = system.processes_by_exact_name("WowT.exe");
        let mut classic_processes = system.processes_by_exact_name("WowClassic.exe");

        let retail = retail_processes.next().is_some() || retail_beta_processes.next().is_some() || retail_ptr_processes.next().is_some();
        let classic = classic_processes.next().is_some();

        // This is JSON.
        println!("{{ \"Retail\": {}, \"Classic\": {} }}", retail, classic);

        thread::sleep(time::Duration::from_secs(2));
    }
}