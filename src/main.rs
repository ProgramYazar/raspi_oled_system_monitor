use embedded_graphics::{mono_font, prelude::*};
use raspi_oled_sysmon::{graphics_display::GraphicDisplay, sysmon::SystemMon};
#[cfg(any(target_arch = "aarch64", target_arch = "aarch64"))]
use rppal::i2c;
use ssd1306::{size::DisplaySize128x64, I2CDisplayInterface};
use std::thread;
use std::time::Duration;

const LCD_ADDRESS: u16 = 0x3c;

#[cfg(target_os = "macos")]
fn main() {
    println!("this binary only compile on raspeberry pi 4 and maybe zero")
}

#[cfg(target_arch = "aarch64")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sys = SystemMon::new();
    let _i2c = i2c::I2c::new()
        .and_then(|mut _i2c| {
            _i2c.set_slave_address(LCD_ADDRESS)?;
            return Ok(_i2c);
        })
        .unwrap();

    let interface = I2CDisplayInterface::new(_i2c);
    let mut display: _ = GraphicDisplay::new(
        interface,
        DisplaySize128x64,
        &mono_font::ascii::FONT_9X15_BOLD,
    );

    loop {
        let cpu_usage = sys.cpu_usage(); // this wait display too
        let mem_usage = sys.memory_usage();
        let cpu_temp = sys.cpu_temp();
        let ip_addr = sys.ip_addr();

        display.clear();
        display.write_text(ip_addr, Point::zero())?;
        display.write_text(
            format!("C: {: >3}%  M: {: >3}%", cpu_usage, mem_usage),
            Point::new(0, 20),
        )?;
        display.write_text(format!("T: {: >3}°C", cpu_temp), Point::new(0, 40))?;
        display.flush()?;

        thread::sleep(Duration::from_secs(5));
    }
}
