use std::thread;
use systemstat::{
    saturating_sub_bytes, BTreeMap, CPULoad, DelayedMeasurement, Duration, IpAddr, Memory, Network,
};

pub fn get_ip_addr(networks: BTreeMap<String, Network>) -> String {
    for na in networks.values().into_iter() {
        for addr in na.addrs.iter() {
            if let IpAddr::V4(ip) = addr.addr {
                let s_addr = ip.to_string();
                if s_addr.starts_with("192") {
                    return s_addr;
                }
            }
        }
    }

    return String::new();
}

pub fn get_cpu_usage(dm: DelayedMeasurement<CPULoad>) -> String {
    thread::sleep(Duration::from_secs(1));
    let cpu = dm.done().unwrap();
    let cpu_usage = ((cpu.user + cpu.system + cpu.nice) * 100.0).round() as i32;
    format!("{: >3}", cpu_usage)
}

pub fn get_memory_usage(mem: Memory) -> String {
    let usage = ((saturating_sub_bytes(mem.total, mem.free).0 as f32 * 100_f32)
        / mem.total.0 as f32)
        .round() as i32;
    format!("{: >3}", usage)
}
