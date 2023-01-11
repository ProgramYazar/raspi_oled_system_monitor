use std::thread;
use std::time::Duration;

use embedded_graphics::{mono_font, prelude::*};

use raspi_old_sysmon::{graphics_display::GraphicDisplay, sysmon::SystemMon};
use rppal::i2c;

use ssd1306::I2CDisplayInterface;

const LCD_ADDRESS: u16 = 0x3c;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sys = SystemMon::new();
    let _i2c = i2c::I2c::new()
        .and_then(|mut _i2c| {
            _i2c.set_slave_address(LCD_ADDRESS)?;
            return Ok(_i2c);
        })
        .unwrap();

    let interface = I2CDisplayInterface::new(_i2c);
    let mut display = GraphicDisplay::new(interface, &mono_font::ascii::FONT_9X15);

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
