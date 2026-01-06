#![no_std]
#![no_main]
#![cfg_attr(test, no_main)]

mod app;
mod display;
mod double_tap_reboot;
mod hardware;

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
    unsafe {
        use core::ptr::addr_of_mut;
        ALLOCATOR.init(addr_of_mut!(HEAP) as *mut u8 as usize, HEAP_SIZE);
    }

    // Initializing the hardware
    let (hw, mut pac_periph, timer_periph) = hardware::init();

    let mut timer = Timer::new(timer_periph, &mut pac_periph.resets, &hw.clocks);
    double_tap_reboot::probe_double_reset(&mut timer);

    // Initializing displays (moving remaining peripherals)
    let mut displays = display::init_displays(pac_periph, &hw);

    // Start the main application loop
    app::run_loop(hw, &mut displays);
}
