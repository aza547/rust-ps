use std::ffi::OsStr;
use std::{thread, time};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

// Handle both Windows and Linux/Wine/Proton process name variations
// Case-insensitive filesystems on windows make this a bit of a puzzle depending
// on how the battle.net client decides to launch the process.
const RETAIL_PROCESSES: &[&str] = &["Wow.exe", "WoW.exe"];
const RETAIL_BETA_PROCESSES: &[&str] = &["WowB.exe", "WoWB.exe"];
const RETAIL_PTR_PROCESSES: &[&str] = &["WowT.exe", "WoWT.exe"];
const CLASSIC_PROCESSES: &[&str] = &["WowClassic.exe", "WoWClassic.exe"];
const CLASSIC_PTR_PROCESSES: &[&str] = &["WowClassicT.exe", "WoWClassicT.exe"];

fn main() {
    let mut system = System::new_all();

    loop {
        system.refresh_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::everything().without_cpu()),
        );

        let retail = RETAIL_PROCESSES.iter()
            .chain(RETAIL_BETA_PROCESSES.iter())
            .chain(RETAIL_PTR_PROCESSES.iter())
            .any(|name| system.processes_by_exact_name(OsStr::new(name)).next().is_some());

        let classic = CLASSIC_PROCESSES.iter()
            .chain(CLASSIC_PTR_PROCESSES.iter())
            .any(|name| system.processes_by_exact_name(OsStr::new(name)).next().is_some());

        // This is JSON.
        println!("{{ \"Retail\": {}, \"Classic\": {} }}", retail, classic);

        thread::sleep(time::Duration::from_secs(2));
    }
}
