#![no_std]
#![no_main]

use defmt_rtt as _;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_graphics::text::{TextStyle, TextStyleBuilder};
use panic_probe as _;
use rp_pico::hal::{clocks::init_clocks_and_plls, gpio::Pins, i2c::I2C, pac, watchdog::Watchdog, Clock, Sio};
use rp_pico::hal::fugit::RateExtU32;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[rp_pico::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    // Core peripherals are not needed here; remove or underscore if retained elsewhere
    let _core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .ok()
        .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // I2C on GPIO4 (SDA) and GPIO5 (SCL)
    let i2c = I2C::i2c1(
        pac.I2C1,
        pins.gpio2.reconfigure(), // SDA 0 2
        pins.gpio3.reconfigure(), // SCL 1 3
        400_u32.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );

    let interface = I2CDisplayInterface::new_custom_address(i2c, 0x3D);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
        .into_buffered_graphics_mode();

    display.init().unwrap();
    // let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On,);
    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On,);
    // let style1 = TextStyleBuilder::new();
    Text::new("RMK Display OK", Point::new(0, 10), style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        cortex_m::asm::wfi();
    }
}