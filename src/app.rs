use crate::display::Displays;
use crate::hardware::Hardware;

pub fn run_loop(_hw: Hardware, _displays: &mut Displays) -> ! {
    loop {
        // Здесь будет ваш основной код
        // Например, обновление дисплеев, чтение сенсоров и т.д.
        _displays.display1.draw(&[1]).ok();
        _displays.display2.draw(&[1]).ok();
        cortex_m::asm::wfi(); // Ожидание прерывания для экономии энергии
    }
}
