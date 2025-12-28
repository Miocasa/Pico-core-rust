// main
#![no_std]
#![no_main]
#![cfg_attr(test, no_main)]

mod hardware;
mod display;
mod app;
mod double_tap_reboot;

use defmt_rtt as _;
use panic_probe as _;

use alloc_cortex_m::CortexMHeap;
use core::mem::MaybeUninit;
use rp_pico::hal::Timer;

// Define the global allocator
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

// Define a static heap buffer (adjust size as needed; 32 KB is a reasonable starting point)
const HEAP_SIZE: usize = 32 * 1024;
static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

#[rp_pico::entry]
fn main() -> ! {
    // Initialize the allocator before any heap usage
    unsafe { ALLOCATOR.init(HEAP.as_mut_ptr() as usize, HEAP_SIZE); }

    // Инициализация аппаратной части
    let (hw, mut pac_periph, timer_periph) = hardware::init();

    let mut timer = Timer::new(timer_periph, &mut pac_periph.resets, &hw.clocks);
    double_tap_reboot::probe_double_reset(&mut timer);

    // Инициализация дисплеев (передаем оставшиеся периферийные устройства)
    let mut displays = display::init_displays(pac_periph, &hw);

    // Запуск основного цикла приложения
    app::run_loop(hw, &mut displays);
}