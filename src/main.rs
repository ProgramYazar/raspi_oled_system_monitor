use std::thread;
use std::time::Duration;

use embedded_graphics::{
    mono_font::{self, ascii::FONT_6X10, MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use raspi_old_sysmon::MockError;

use rppal::i2c;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

use raspi_old_sysmon::SystemMon;
use ssd1306::size::DisplaySize128x64;

const LCD_ADDRESS: u16 = 0x3c;

struct GraphicDisplay<'a, DI> {
    display: Ssd1306<DI, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
    text_style: MonoTextStyle<'a, BinaryColor>,
}

impl<'a, DI> GraphicDisplay<'a, DI>
where
    DI: WriteOnlyDataCommand,
{
    pub fn new(i2c_interface: DI, font: &'a MonoFont) -> Self {
        let mut display = Ssd1306::new(
            i2c_interface,
            DisplaySize128x64,
            ssd1306::rotation::DisplayRotation::Rotate0,
        )
        .into_buffered_graphics_mode();
        display.init().unwrap();
        display.clear();

        Self {
            display,
            text_style: MonoTextStyleBuilder::new()
                .font(&font)
                .text_color(BinaryColor::On)
                .build(),
        }
    }
    pub fn clear(&mut self) {
        self.display.clear();
    }
    pub fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.display.flush() {
            Ok(()) => Ok(()),
            Err(e) => Err(Box::new(MockError::new(format!(
                "display flush error: {:?}",
                e
            )))),
        }
    }

    pub fn write_text(
        &mut self,
        text: String,
        pos: Point,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        match Text::new(text.as_str(), pos, text_style).draw(&mut self.display) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(MockError::new(format!("error: {:?}", e)))),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sys = SystemMon::new();
    let _i2c = i2c::I2c::new()
        .and_then(|mut _i2c| {
            _i2c.set_slave_address(LCD_ADDRESS)?;
            return Ok(_i2c);
        })
        .unwrap();

    let interface = I2CDisplayInterface::new(_i2c);
    let mut display = GraphicDisplay::new(interface, &mono_font::ascii::FONT_6X10);

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
