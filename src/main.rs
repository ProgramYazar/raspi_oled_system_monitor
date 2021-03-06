use embedded_graphics::fonts::Font8x16;
use embedded_graphics::{
    drawable::Drawable, fonts::Text, pixelcolor::BinaryColor, prelude::Point,
    style::TextStyleBuilder,
};
use rppal::i2c::I2c;
use ssd1306::{displaysize::DisplaySize128x64, mode::GraphicsMode, Builder, I2CDIBuilder};
use std::time::Duration;

use std::error::Error;
use std::thread;
use systemstat::{Platform, System};
mod lib;
use lib::*;

fn main() {
    loop {
        if let Err(_) = run() {
            thread::sleep(Duration::from_secs(5));
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let sys = System::new();

    let mut i2c = I2c::new()?;
    i2c.set_slave_address(0x3c).unwrap();

    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: GraphicsMode<_, _> = Builder::new()
        .size(DisplaySize128x64)
        .connect(interface)
        .into();

    disp.init().unwrap();

    let text_style = TextStyleBuilder::new(Font8x16)
        .text_color(BinaryColor::On)
        .build();

    loop {
        let cpu_usage = get_cpu_usage(sys.cpu_load_aggregate()?); // this wait display too
        let mem_usage = get_memory_usage(sys.memory()?);
        let cpu_temp = sys.cpu_temp()?.round() as i32;

        disp.clear();
        Text::new(get_ip_addr(sys.networks().unwrap()).as_str(), Point::zero())
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();

        Text::new(
            format!("C: {}%  M: {}%", cpu_usage, mem_usage).as_str(),
            Point::new(0, 20),
        )
        .into_styled(text_style)
        .draw(&mut disp)
        .unwrap();

        Text::new(format!("T: {}°C", cpu_temp).as_str(), Point::new(0, 40))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();
        disp.flush().unwrap();
    }

    // disp.release();

    // Ok(())
}
