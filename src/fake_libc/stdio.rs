use core::{ffi::{c_void, c_int, c_char}, alloc::{GlobalAlloc, Layout}, fmt, str::Bytes};

mod c {
    use core::ffi::{c_int, c_void};

    extern "C" {
        pub fn putchar(ch: c_int) -> c_int;
    }
}

pub fn putchar(c: u8) -> u8 {
    unsafe { c::putchar(c as c_int) as u8 }
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
