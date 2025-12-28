use rp_pico::hal;
// hardware
use rp_pico::hal::{
    clocks::{init_clocks_and_plls, ClocksManager},
    pac,
    watchdog::Watchdog,
    Sio,
};

pub struct Hardware {
    pub clocks: ClocksManager,
}

pub struct RemainingPeripherals {
    pub io_bank0: pac::IO_BANK0,
    pub pads_bank0: pac::PADS_BANK0,
    pub i2c1: pac::I2C1,
    pub resets: pac::RESETS,
    pub gpio_bank0: hal::sio::SioGpioBank0,
}

pub fn init() -> (Hardware, RemainingPeripherals, pac::TIMER) {
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
    let gpio_bank0 = sio.gpio_bank0;

    let timer = pac.TIMER;

    let remaining = RemainingPeripherals {
        io_bank0: pac.IO_BANK0,
        pads_bank0: pac.PADS_BANK0,
        i2c1: pac.I2C1,
        resets: pac.RESETS,
        gpio_bank0,
    };

    (Hardware { clocks }, remaining, timer)
}
