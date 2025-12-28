use crate::hardware::{Hardware, RemainingPeripherals};
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use rp_pico::hal::fugit::RateExtU32;
use rp_pico::hal::{gpio::Pins, i2c::I2C, pac, Clock};
use rp_pico::pac::RESETS;
use ssd1306::mode::BufferedGraphicsMode;
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

type DisplayType =
    Ssd1306<I2CInterface<SharedI2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;

static I2C_BUS: Mutex<RefCell<Option<I2cType>>> = Mutex::new(RefCell::new(None));

pub struct Displays {
    pub display1: DisplayType,
    pub display2: DisplayType,
}

// Обертка для разделяемого доступа к I2C
pub struct SharedI2c;

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

pub fn init_displays(periph: RemainingPeripherals, hw: &Hardware) -> Displays {
    let mut resets: RESETS = periph.resets;

    let pins = Pins::new(
        periph.io_bank0,
        periph.pads_bank0,
        periph.gpio_bank0,
        &mut resets,
    );

    // Создаем I2C шину на GPIO2 (SDA) и GPIO3 (SCL)
    let i2c = I2C::i2c1(
        periph.i2c1,
        pins.gpio2.reconfigure(),
        pins.gpio3.reconfigure(),
        400_u32.kHz(),
        &mut resets,
        hw.clocks.system_clock.freq(),
    );

    // Помещаем шину в глобальный Mutex
    cortex_m::interrupt::free(|cs| {
        I2C_BUS.borrow(cs).replace(Some(i2c));
    });

    // Первый дисплей (адрес 0x3D)
    let interface1 = I2CDisplayInterface::new_custom_address(SharedI2c, 0x3D);
    let mut display1 = Ssd1306::new(interface1, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display1.init().unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    Text::new("RMK Display 1", Point::new(0, 10), style)
        .draw(&mut display1)
        .unwrap();
    Text::new("OK", Point::new(0, 30), style)
        .draw(&mut display1)
        .unwrap();

    display1.flush().unwrap();

    // Второй дисплей (адрес 0x3C)
    let interface2 = I2CDisplayInterface::new_custom_address(SharedI2c, 0x3C);
    let mut display2 = Ssd1306::new(interface2, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display2.init().unwrap();

    Text::new("RMK Display 2", Point::new(0, 15), style)
        .draw(&mut display2)
        .unwrap();
    Text::new("OK", Point::new(0, 35), style)
        .draw(&mut display2)
        .unwrap();

    display2.flush().unwrap();

    Displays { display1, display2 }
}
