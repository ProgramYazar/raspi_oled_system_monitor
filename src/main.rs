use std::thread;
use std::time::Duration;

use embedded_graphics::fonts::Font8x16;
use embedded_graphics::{
    drawable::Drawable, fonts::Text, pixelcolor::BinaryColor, prelude::Point,
    style::TextStyleBuilder,
};
use rppal::i2c::I2c;
use ssd1306::{displaysize::DisplaySize128x64, mode::GraphicsMode, Builder, I2CDIBuilder};
mod lib;
use lib::{MockError, SystemMon};

const LCD_ADDRESS: u16 = 0x3c;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sys = SystemMon::new();

    let lcd_interface =
        match I2c::new().and_then(|mut i2c| match i2c.set_slave_address(LCD_ADDRESS) {
            Ok(_) => Ok(i2c),
            Err(e) => Err(e),
        }) {
            Ok(i2c) => I2CDIBuilder::new().init(i2c),
            Err(e) => {
                return Err(Box::new(MockError::new(format!(
                    "lcd interface init problem<i2c>: {:?}",
                    e
                ))))
            }
        };

    let mut disp: GraphicsMode<_, _> = Builder::new()
        .size(DisplaySize128x64)
        .connect(lcd_interface)
        .into();

    if let Err(e) = disp.init() {
        let err_message = format!("display init problem: {:?}", e);
        return Err(Box::new(MockError::new(err_message)));
    }

    let text_style = TextStyleBuilder::new(Font8x16)
        .text_color(BinaryColor::On)
        .build();

    let mut write_to_lcd = |text: String, pos: Point| match Text::new(text.as_str(), pos)
        .into_styled(text_style)
        .draw(&mut disp)
    {
        Ok(_) => (),
        Err(e) => println!("error: {:?}", e),
    };

    loop {
        let cpu_usage = sys.cpu_usage(); // this wait display too
        let mem_usage = sys.memory_usage();
        let cpu_temp = sys.cpu_temp();
        let ip_addr = sys.ip_addr();

        //disp.clear();
        write_to_lcd(ip_addr, Point::zero());
        // write_to_lcd(
        //     format!("C: {: >3}%  M: {: >3}%", cpu_usage, mem_usage),
        //     Point::new(0, 20),
        // );
        write_to_lcd(format!("T: {: >3}°C", cpu_temp), Point::new(0, 40));

        //disp.flush().unwrap();
        thread::sleep(Duration::from_secs(5));
    }
}
