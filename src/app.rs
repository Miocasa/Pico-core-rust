use crate::display::Displays;
use crate::hardware::Hardware;

pub fn run_loop(_hw: Hardware, _displays: &mut Displays) -> ! {
    loop {
        // Your main code will go here
        // For example, updating displays, reading sensors, etc.
        _displays.display1.draw(&[1]).ok();
        _displays.display2.draw(&[1]).ok();
        cortex_m::asm::wfi(); // Wait for interrupt to save power
    }
}
