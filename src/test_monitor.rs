use embedded_graphics::{mono_font, prelude::*};
use raspi_oled_sysmon::sysmon::SystemMon;
use raspi_oled_sysmon::test_display::TestDisplay;

use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sys = SystemMon::new();

    let mut display: _ = TestDisplay::new(&mono_font::ascii::FONT_9X15);

    loop {
        let cpu_usage = sys.cpu_usage(); // this wait display too
        let mem_usage = sys.memory_usage();
        let cpu_temp = sys.cpu_temp();
        let ip_addr = sys.ip_addr();

        println!("C: {: >3}%  M: {: >3}%", cpu_usage, mem_usage);
        println!("T: {: >3}°C", cpu_temp);
        println!();

        // display.clear();
        display.write_text(ip_addr, Point::zero())?;
        display.write_text(
            format!("C: {: >3}%  M: {: >3}%", cpu_usage, mem_usage),
            Point::new(0, 20),
        )?;
        display.write_text(format!("T: {: >3}°C", cpu_temp), Point::new(0, 40))?;
        // display.flush()?;

        thread::sleep(Duration::from_secs(5));
    }
}
