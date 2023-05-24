#![feature(lang_items)]
#![no_std]

extern crate alloc;
use core::{time::Duration, mem::transmute};

use alloc::{
    boxed::Box,
    string::String
};

extern crate embedded_alloc;
use embedded_alloc::Heap;
use logimpl::emulated;

use crate::logimpl::serial::Logger;
use slint::{
    Timer,
    TimerMode
};

use crate::hw::serial;

mod fake_libc;
mod hw;
mod panic;
mod logimpl;

#[global_allocator]
//static GLOBAL: MyAllocator = MyAllocator;
static HEAP: Heap = Heap::empty();

/*
#[no_mangle]
pub extern fn malloc(size: usize) -> *mut void {
    unsafe {
        let mut l = Layout::from_size_align_unchecked(size, 1);
        HEAP.alloc(l)
    }
}
#[no_mangle]
pub extern fn free(ptr: *mut void) {
    unsafe {
        let mut l = Layout::from_size_align_unchecked(size, 1);
        HEAP.dealloc(l)
    }
}
#[no_mangle]
pub extern fn realloc(ptr: *mut void, new_size: usize) -> *mut void {

}*/

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

mod fb;
mod platform;
mod surfaces;

#[cfg(not(feature = "emulator"))]
pub fn init_platform() {
    use platform::*;
    let surface = unsafe {
        surfaces::vga320x200_256c::Surface::new()
    };
    slint::platform::set_platform(Box::new(Platform::new(surface))).unwrap();
}

#[cfg(feature = "emulator")]
mod emulator;

#[cfg(feature = "emulator")]
pub fn init_platform() {
    slint::platform::set_platform(Box::<emulator::Platform>::default()).unwrap();
}

slint::include_modules!();

pub fn diplay_slint() {
    init_platform();

    let ui = Demo::new().unwrap();

    let weak_ui = ui.as_weak();
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_millis(35), move || {
        let ui = weak_ui.upgrade().unwrap();

        ui.set_tick(hw::timer_tick() as f32);
    });

    ui.run().unwrap();
}

#[derive(Debug)]
#[repr(C, align(4))]
pub struct GraphicsHeader {
    flag0: u32,
    w: u32,
    h: u32,
    bpp: u32,
}

#[derive(Debug)]
#[repr(C, align(4))]
pub struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,

    flag0: u32,
    flag1: u32,
    flag2: u32,
    flag3: u32,
    flag4: u32,

    graphics: GraphicsHeader,
}

struct R<'a>(pub &'a Logger);
unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    transmute::<R<'b>, R<'static>>(r)
}

#[cfg(feature = "emulator")]
static emulated_logger: emulated::Logger = emulated::Logger;

#[no_mangle]
pub extern fn rust_main(header: MultibootHeader) {
    use core::fmt::Write;

    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024 * 1024 * 64;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    #[cfg(not(feature = "emulator"))]
    let logger: Logger = Logger::new(serial::Port::COM1).unwrap();
    #[cfg(not(feature = "emulator"))]
    unsafe { extend_lifetime(R(&logger)).0.init().unwrap() };

    #[cfg(feature = "emulator")]
    emulated_logger.init().unwrap();

    log::debug!("debug: gogadoda");


    let mut host_stderr = fake_libc::stdio::Stdout::new();

    writeln!(host_stderr, "header: {:?}", header).ok();

    let s = String::from("asasa");
    writeln!(host_stderr, "after string aloc: {}", s.as_str()).ok();

    let b: Option<Box<String>> = Some(Box::new(String::from("ababba")));
    if let Some(b) = b {
        writeln!(host_stderr, "b: {}", b.as_str()).ok();
    }

    diplay_slint();
}
