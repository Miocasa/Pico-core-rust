#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use defmt_rtt as _;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use panic_probe as _;
use rp_pico::hal::{
    clocks::init_clocks_and_plls,
    gpio::Pins,
    i2c::I2C,
    pac,
    watchdog::Watchdog,
    Clock,
    Sio
};
use rp_pico::hal::fugit::RateExtU32;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

type I2cType = I2C<
    pac::I2C1,
    (
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio2,
            rp_pico::hal::gpio::FunctionI2c,
            rp_pico::hal::gpio::PullUp,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio3,
            rp_pico::hal::gpio::FunctionI2c,
            rp_pico::hal::gpio::PullUp,
        >,
    ),
>;

static I2C_BUS: Mutex<RefCell<Option<I2cType>>> = Mutex::new(RefCell::new(None));

// Обертка для разделяемого доступа к I2C
struct SharedI2c;

impl embedded_hal::i2c::ErrorType for SharedI2c {
    type Error = rp_pico::hal::i2c::Error;
}

impl embedded_hal::i2c::I2c for SharedI2c {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut i2c) = I2C_BUS.borrow(cs).borrow_mut().as_mut() {
                i2c.transaction(address, operations)
            } else {
                Err(rp_pico::hal::i2c::Error::Abort(0))
            }
        })
    }
}

#[rp_pico::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
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

    // Создаем I2C шину на GPIO2 (SDA) и GPIO3 (SCL)
    let i2c = I2C::i2c1(
        pac.I2C1,
        pins.gpio2.reconfigure(), // SDA
        pins.gpio3.reconfigure(), // SCL
        400_u32.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );

    // Помещаем шину в глобальный Mutex
    cortex_m::interrupt::free(|cs| {
        I2C_BUS.borrow(cs).replace(Some(i2c));
    });

    // Первый дисплей (адрес 0x3D)
    let interface1 = I2CDisplayInterface::new_custom_address(SharedI2c, 0x3D);
    let mut display1 = Ssd1306::new(
        interface1,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
        .into_buffered_graphics_mode();

    display1.init().unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    Text::new("RMK Display 1", Point::new(0, 20), style)
        .draw(&mut display1)
        .unwrap();
    Text::new("OK", Point::new(0, 35), style)
        .draw(&mut display1)
        .unwrap();

    display1.flush().unwrap();

    // Второй дисплей (адрес 0x3C)
    let interface2 = I2CDisplayInterface::new_custom_address(SharedI2c, 0x3C);
    let mut display2 = Ssd1306::new(
        interface2,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
        .into_buffered_graphics_mode();

    display2.init().unwrap();

    Text::new("RMK Display 2", Point::new(0, 20), style)
        .draw(&mut display2)
        .unwrap();
    Text::new("OK", Point::new(0, 35), style)
        .draw(&mut display2)
        .unwrap();

    display2.flush().unwrap();

    loop {
        cortex_m::asm::wfi();
    }
}