use core::{ffi::{c_void, c_char}, fmt};

extern "C" {
    fn __panic__(context: *const c_void, cb: fn(*const c_void, *mut c_void));
    fn __panic_push__(panic_handle: *mut c_void, message: *const c_char, msize: usize);
}

/// Panic output
struct Panout(*mut c_void);

impl Panout {
    pub fn new(panic_handle: *mut c_void) -> Self {
        return Self(panic_handle)
    }
}

impl fmt::Write for Panout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            __panic_push__(self.0, s.as_ptr() as *const c_char, s.len());
        }
        Ok(())
    }
}

#[cfg(not(test))]
mod no_test {
    use core::panic::PanicInfo;
    use core::ffi::c_void;

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        unsafe {
            super::__panic__(info as *const PanicInfo as *const c_void, |ctx, handle| {
                use core::fmt::Write;
                let info = &*(ctx as *const PanicInfo);
                let mut host_stderr = super::Panout::new(handle);
                writeln!(host_stderr, "{}", info).ok();
            });
        }
        loop {}
    }

    #[lang = "eh_personality"]
    #[no_mangle]
    extern fn eh_personality() {}
}

