use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::Point,
    text::Text,
    Drawable,
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};

use crate::mock_error::MockError;

pub struct GraphicDisplay<'a, DI, DS: DisplaySize> {
    display: Ssd1306<DI, DS, BufferedGraphicsMode<DS>>,
    text_style: MonoTextStyle<'a, BinaryColor>,
}

impl<'a, DI, DS> GraphicDisplay<'a, DI, DS>
where
    DI: WriteOnlyDataCommand, /* i2c interface*/
    DS: DisplaySize,
{
    pub fn new(i2c_interface: DI, size: DS, font: &'a MonoFont) -> Self {
        let mut display = Ssd1306::new(
            i2c_interface,
            size,
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
        match Text::new(text.as_str(), pos, self.text_style).draw(&mut self.display) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(MockError::new(format!("error: {:?}", e)))),
        }
    }
}
