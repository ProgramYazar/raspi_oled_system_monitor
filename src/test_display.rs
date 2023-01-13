use embedded_graphics::{
    mock_display::MockDisplay,
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
    Drawable,
};

use crate::mock_error::MockError;

pub struct TestDisplay<'a> {
    display: MockDisplay<BinaryColor>,
    text_style: MonoTextStyle<'a, BinaryColor>,
}

impl<'a> TestDisplay<'a> {
    pub fn new(font: &'a MonoFont) -> Self {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_out_of_bounds_drawing(true);
        display.set_allow_overdraw(true);
        Self {
            display,
            text_style: MonoTextStyleBuilder::new()
                .font(&font)
                .text_color(BinaryColor::On)
                .build(),
        }
    }
    pub fn clear(&mut self) {
        // self.display.clear();
    }
    pub fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // match self.display.flush() {
        //     Ok(()) => Ok(()),
        //     Err(e) => Err(Box::new(MockError::new(format!(
        //         "display flush error: {:?}",
        //         e
        //     )))),
        // }
        Ok(())
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
