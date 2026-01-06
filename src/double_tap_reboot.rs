//! Enable bootsel via double reset.
// #![no_std]

use core::ptr::{addr_of, addr_of_mut};
use core::{arch::asm, mem::MaybeUninit};
use embedded_hal::delay::DelayNs;
use rp_pico::hal::rom_data::reset_to_usb_boot;
use rp_pico::hal::Timer;

pub fn probe_double_reset(delay: &mut Timer) {
    #[link_section = ".uninit"]
    static mut FLAG: MaybeUninit<u32> = MaybeUninit::uninit();

    let mut flag: u32;
    unsafe {
        asm!(
        "ldr {flag}, [{addr}]",
        addr = in(reg) addr_of!(FLAG) as *const u8,
        flag = out(reg) flag,
        );
    }

    if flag == 0x0B0075E1 {
        unsafe { addr_of_mut!(FLAG).write(MaybeUninit::new(0)) };
        delay.delay_ms(500);
        // trigger bootsel
        reset_to_usb_boot(0, 0);
        #[allow(clippy::empty_loop)]
        loop {}
    } else {
        let value = 0x0B0075E1;
        unsafe {
            asm!(
            "str {value}, [{addr}]",
            addr = in(reg) addr_of!(FLAG) as *const u8,
            value = in(reg) value,
            );
        }
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        delay.delay_ms(500);
        let value = 0;
        unsafe {
            asm!(
            "str {value}, [{addr}]",
            addr = in(reg) addr_of!(FLAG) as *const u8,
            value = in(reg) value,
            );
        }
    }
}
