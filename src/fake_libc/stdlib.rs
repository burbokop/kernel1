use core::{ffi::{c_void, c_int, c_char}, alloc::{GlobalAlloc, Layout}, fmt, str::Bytes};

mod c {
    use core::ffi::{c_int, c_void};

    extern "C" {
        pub fn malloc(size: usize) -> *mut c_void;
        pub fn free(ptr: *mut c_void);
    }
}



struct Alloc;
unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        c::malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        c::free(ptr as *mut c_void)
    }
}
