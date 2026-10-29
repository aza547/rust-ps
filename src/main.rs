use std::ffi::OsStr;
use std::{thread, time};use sysinfo::{Process, ProcessRefreshKind, RefreshKind, System};

// Handle both Windows and Linux/Wine/Proton process name variations
// Case-insensitive filesystems on windows make this a bit of a puzzle depending
// on how the battle.net client decides to launch the process.
const RETAIL_PROCESSES: &[&str] = &["Wow.exe", "WoW.exe"];
const RETAIL_BETA_PROCESSES: &[&str] = &["WowB.exe", "WoWB.exe"];
const RETAIL_PTR_PROCESSES: &[&str] = &["WowT.exe", "WoWT.exe"];
const CLASSIC_PROCESSES: &[&str] = &["WowClassic.exe", "WoWClassic.exe"];
const CLASSIC_PTR_PROCESSES: &[&str] = &["WowClassicT.exe", "WoWClassicT.exe"];

// The forever client shares the retail executable name, so it can only be
// told apart by the folder it is launched from.
const FOREVER_FOLDER: &str = "_forever_";

fn is_forever(process: &Process) -> bool {
    process
        .exe()
        .and_then(|exe| exe.parent())
        .and_then(|dir| dir.file_name())
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case(FOREVER_FOLDER))
}

fn main() {
    let mut system = System::new_all();

    loop {
        system.refresh_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::everything().without_cpu()),
        );

        let retail_processes = || {
            RETAIL_PROCESSES.iter()
                .chain(RETAIL_BETA_PROCESSES.iter())
                .chain(RETAIL_PTR_PROCESSES.iter())
                .flat_map(|name| system.processes_by_exact_name(OsStr::new(name)))
        };

        let retail = retail_processes().any(|p| !is_forever(p));
        let forever = retail_processes().any(is_forever);

        let classic = CLASSIC_PROCESSES.iter()
            .chain(CLASSIC_PTR_PROCESSES.iter())
            .any(|name| system.processes_by_exact_name(OsStr::new(name)).next().is_some());

        // This is JSON.
        println!(
            "{{ \"Retail\": {}, \"Classic\": {}, \"Forever\": {} }}",
            retail, classic, forever
        );

        thread::sleep(time::Duration::from_secs(2));
    }
}
