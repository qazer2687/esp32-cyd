use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    mono_font::{ascii::FONT_9X18, MonoTextStyle},
    pixelcolor::{Rgb565, BinaryColor},
	primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    prelude::*,
    text::renderer::CharacterStyle,
    text::{Text, Alignment},
};

use esp_idf_svc::hal::{gpio, prelude::Peripherals};

use esp_idf_hal::{
    delay::Ets,
    spi::{config::{Config, DriverConfig}, Dma, SpiDeviceDriver}, units::MegaHertz,
};

use mipidsi::Builder;

use std::error::Error;

use esp_idf_sys as _;

fn main() -> Result<(), Box<dyn Error>> {
    esp_idf_sys::link_patches();

	// get peripherals and pins
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let rst = gpio::PinDriver::output(pins.gpio4)?;
    let dc = gpio::PinDriver::output(pins.gpio2)?;

    // use built in delay provider
    let mut delay = Ets;

    // configure pins
    let sclk = pins.gpio14;
    let spi = peripherals.spi2;
    let sdo = pins.gpio13;
    let sdi = pins.gpio12;
    let cs = pins.gpio15;

	// create the SPI interface
    let di = SPIInterfaceNoCS::new(
        SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Some(sdi),
            Some(cs),
            &DriverConfig::new().dma(Dma::Disabled),
            &Config::new().baudrate(MegaHertz(40).into()),
        )?,
        dc,
    );

	// configure display using mipidsi's builder
    let mut display = Builder::ili9341_rgb565(di)
        .with_color_order(mipidsi::ColorOrder::Rgb)
        .with_orientation(mipidsi::options::Orientation::LandscapeInverted(true))
        .init(&mut delay, Some(rst))
        .map_err(|_| Box::<dyn Error>::from("display init"))?;

    // configure backlight
    let mut bl = gpio::PinDriver::output(pins.gpio21)?;
    bl.set_high()?;
    core::mem::forget(bl);

	// clear display to black
    display
        .clear(Rgb565::BLACK)
        .map_err(|_| Box::<dyn Error>::from("clear display"))?;

    let mut style = MonoTextStyle::new(&FONT_9X18, Rgb565::WHITE);
	let mut offset_y: i32 = 20; 
	let offset_x = 20;
	let mut line_number = 1;
	let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::WHITE)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
	let thick_stroke = PrimitiveStyle::with_stroke(Rgb565::YELLOW, 1);

	// draw box around display
	display
        .bounding_box()
        .into_styled(border_stroke)
        .draw(&mut display)
		.map_err(|_| Box::<dyn Error>::from("failed to draw border"))?;

	style.set_text_color(Some(Rgb565::RED));
    Text::new("This is red!", Point::new(offset_x, offset_y), style)
        .draw(&mut display)
        .map_err(|_| Box::<dyn Error>::from("failed to draw text"))?;
	offset_y += style.font.character_size.height as i32;

	
	style.set_text_color(Some(Rgb565::BLUE));
	Text::new("This is blue!", Point::new(offset_x, offset_y), style)
        .draw(&mut display)
        .map_err(|_| Box::<dyn Error>::from("failed to draw text"))?;
	offset_y += style.font.character_size.height as i32;

	style.set_text_color(Some(Rgb565::GREEN));
	Text::new("This is green!", Point::new(offset_x, offset_y), style)
		.draw(&mut display)
		.map_err(|_| Box::<dyn Error>::from("failed to draw text"))?;
	offset_y += style.font.character_size.height as i32;
	
	// test this, no idea what color this is or whether it works
	style.set_text_color(Some(Rgb565::new(0x8F, 0x41, 0x0E)));
	Text::new("This is purple!", Point::new(offset_x, offset_y), style)
		.draw(&mut display)
		.map_err(|_| Box::<dyn Error>::from("failed to draw text"))?;
	offset_y += style.font.character_size.height as i32;

	Circle::new(Point::new(offset_x, offset_y), 10)
        .into_styled(thick_stroke)
        .draw(&mut display)
		.map_err(|_| Box::<dyn Error>::from("failed to draw circle"))?;

	style.set_text_color(Some(Rgb565::YELLOW));
	Text::new("<- circle", Point::new(offset_x + 20, offset_y + 10), style)
		.draw(&mut display)
		.map_err(|_| Box::<dyn Error>::from("failed to draw text"))?;

	offset_y += style.font.character_size.height as i32;

	style.set_text_color(Some(Rgb565::WHITE));

	let text = "im in the center";
    Text::with_alignment(
        text,
        display.bounding_box().center() + Point::new(0, 15),
        style,
        Alignment::Center,
    )
    .draw(&mut display)
	.map_err(|_| Box::<dyn Error>::from("failed to draw text"))?;
	
    Ok(())
}