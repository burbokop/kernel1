
mod c {
    use core::ffi::{c_int, c_void};

    extern "C" {
        pub fn malloc(size: usize) -> *mut c_void;
        pub fn free(ptr: *mut c_void);
        pub fn putchar(ch: c_int) -> c_int;
    }
}

pub fn putchar(c: u8) -> u8 {
    unsafe { c::putchar(c as c_int) as u8 }
}

use core::{ffi::{c_void, c_int}, alloc::{GlobalAlloc, Layout}, fmt};

struct Alloc;
unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        c::malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        c::free(ptr as *mut c_void)
    }
}
pub struct Stdout;

impl Stdout {
    pub fn new() -> Self {
        return Self {}
    }
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                c::putchar(byte as c_int);
            }
        }
        Ok(())
    }
}

pub type Stderr = Stdout;
