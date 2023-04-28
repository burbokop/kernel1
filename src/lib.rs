#![feature(lang_items)]
#![no_std]

extern crate alloc;
use core::time::Duration;

use alloc::{boxed::Box, string::{String, ToString}, format};

extern crate embedded_alloc;
use embedded_alloc::Heap;
use slint::{Timer, TimerMode};

mod cstd;
mod hw;

#[global_allocator]
//static GLOBAL: MyAllocator = MyAllocator;
static HEAP: Heap = Heap::empty();


#[cfg(not(feature = "std"))] // needed for `cargo test --features std`
mod no_std {
    extern crate critical_section;
    use self::critical_section::RawRestoreState;
    struct MyCriticalSection;
    critical_section::set_impl!(MyCriticalSection);

    unsafe impl critical_section::Impl for MyCriticalSection {
        unsafe fn acquire() -> RawRestoreState {}
        unsafe fn release(_: RawRestoreState) {}
    }
}

slint::include_modules!();

mod fb;

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
mod platform;

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub fn init_platform() {
    // TODO
    //slint::platform::set_platform(Box::<Platform>::default()).unwrap();
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
mod emulator;

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
pub fn init_platform() {
    slint::platform::set_platform(Box::<emulator::Platform>::default()).unwrap();
}

pub fn diplay_slint() {
    init_platform();

    let ui = Demo::new().unwrap();

    ui.set_firmware_vendor("burbokop".to_string().into());
    ui.set_firmware_version(
        format!("{}.{:02}", 0, 0).into(),
    );
    ui.set_uefi_version("1.0.0".to_string().into());

    ui.set_secure_boot(false);

    let weak_ui = ui.as_weak();
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_millis(35), move || {
        let ui = weak_ui.upgrade().unwrap();

        ui.set_timer_tick(hw::timer_tick() as f32);
        ui.set_pit(hw::ttt() as f32);
    });

    ui.run().unwrap();
}

#[derive(Debug)]
#[repr(C, align(4))]
pub struct graphics_header {
    flag0: u32,
    w: u32,
    h: u32,
    bpp: u32,
}

#[derive(Debug)]
#[repr(C, align(4))]
pub struct multiboot_header {
    magic: u32,
    flags: u32,
    checksum: u32,

    flag0: u32,
    flag1: u32,
    flag2: u32,
    flag3: u32,
    flag4: u32,

    graphics: graphics_header,
}

#[no_mangle]
pub extern fn rust_main(header: multiboot_header) {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024 * 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    use core::fmt::Write;
    let mut host_stderr = cstd::Stdout::new();

    writeln!(host_stderr, "header: {}", header).ok();

    let s = String::from("asasa");
    writeln!(host_stderr, "after string aloc: {}", s.as_str()).ok();

    let b: Option<Box<String>> = Some(Box::new(String::from("ababba")));
    if let Some(b) = b {
        writeln!(host_stderr, "b: {}", b.as_str()).ok();
    }
    diplay_slint();
}

#[no_mangle]
pub extern fn aaa() -> usize { 1234 }


#[cfg(not(test))]
mod panic {
    use core::panic::PanicInfo;
    use crate::cstd::Stderr;

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        use core::fmt::Write;
        let mut host_stderr = Stderr::new();
        writeln!(host_stderr, "{}", info).ok();
        loop {}
    }

    #[lang = "eh_personality"]
    #[no_mangle]
    pub extern fn eh_personality() {}
}
