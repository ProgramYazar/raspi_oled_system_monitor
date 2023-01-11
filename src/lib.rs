use std::{fmt, thread};
use systemstat::{saturating_sub_bytes, Duration, IpAddr, Platform, System};

pub struct SystemMon {
    plat: System,
}

impl SystemMon {
    pub fn new() -> SystemMon {
        Self {
            plat: systemstat::System::new(),
        }
    }

    pub fn ip_addr(&self) -> String {
        if let Ok(networks) = self.plat.networks() {
            for na in networks.values() {
                for addr in na.addrs.iter() {
                    if let IpAddr::V4(ip) = addr.addr {
                        let s_addr = ip.to_string();
                        if s_addr.starts_with("192") {
                            return s_addr;
                        }
                    }
                }
            }
        }
        String::new()
    }

    pub fn cpu_usage(&self) -> u32 {
        if let Ok(cpu) = self.plat.cpu_load_aggregate().and_then(|cla| {
            thread::sleep(Duration::from_millis(500));
            cla.done()
        }) {
            ((cpu.user + cpu.system + cpu.nice) * 100.0).round() as u32
        } else {
            0
        }
    }

    pub fn memory_usage(&self) -> u32 {
        if let Ok(mem) = self.plat.memory() {
            ((saturating_sub_bytes(mem.total, mem.free).0 as f32 * 100_f32) / mem.total.0 as f32)
                .round() as u32
            //format!("{: >3}", usage)
        } else {
            0
        }
    }

    pub fn cpu_temp(&self) -> u32 {
        match self.plat.cpu_temp() {
            Ok(val) => val.round() as u32,
            Err(_) => 0,
        }
    }
}

#[derive(Debug)]
pub struct MockError {
    details: String,
}

impl MockError {
    pub fn new(msg: String) -> Self {
        Self { details: msg }
    }
}

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for MockError {
    fn description(&self) -> &str {
        &self.details
    }
}
