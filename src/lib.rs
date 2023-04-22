#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn rust_main() {}

#[no_mangle]
pub extern fn aaa() -> usize { 1234 }

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}


