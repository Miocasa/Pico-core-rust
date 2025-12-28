use crate::hardware::Hardware;
use crate::display::Displays;

pub fn run_loop(_hw: Hardware, _displays: &mut Displays) -> ! {
    loop {
        // Здесь будет ваш основной код
        // Например, обновление дисплеев, чтение сенсоров и т.д.
        _displays.display1.draw(&[1]);
        _displays.display2.draw(&[1]);
        cortex_m::asm::wfi(); // Ожидание прерывания для экономии энергии
    }
}