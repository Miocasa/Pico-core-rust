#![no_std]
#![no_main]
#![cfg_attr(test, no_main)]

mod hardware;
mod display;
mod app;

use defmt_rtt as _;
use panic_probe as _;

#[rp_pico::entry]
fn main() -> ! {
    // Инициализация аппаратной части
    let (hw, pac_periph) = hardware::init();

    // Инициализация дисплеев (передаем оставшиеся периферийные устройства)
    let mut displays = display::init_displays(pac_periph, &hw);

    // Запуск основного цикла приложения
    app::run_loop(hw, &mut displays);
}