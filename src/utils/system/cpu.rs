use std::thread;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub fn get_total_average() -> f32 {
    let mut s =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    s.refresh_cpu();

    let cpus = s.cpus();

    let cpu_sum = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();

    cpu_sum / cpus.len() as f32
}
