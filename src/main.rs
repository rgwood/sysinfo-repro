use sysinfo::{CpuRefreshKind, CpuExt, System, SystemExt};

fn main() {
    let mut sys = System::new();
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());

    for cpu in sys.cpus() {
        println!("{: >6}: {}", cpu.name(), cpu.cpu_usage());
    }
}
