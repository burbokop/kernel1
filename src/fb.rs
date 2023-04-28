
use core::{ffi::c_void, mem::transmute};

pub struct FrameBuffer {
    data: *mut c_void,
    pitch: usize,      // len in bytes of one line
    depth: usize,      // bits per pixel
    w: usize,          // pixels in row
    h: usize,          // pixels in column
}

impl FrameBuffer {
    /// unsafe because requere "VGA 320x200 256 color" mode to be enabled in BIOS
    pub unsafe fn vga320x200_256color() -> Self {
        Self {
            data: transmute(0xA0000 as usize),
            pitch: 320,
            depth: 8,
            w: 320,
            h: 200,
        }
    }

    pub unsafe fn pixel_addr(&mut self, x: usize, y: usize) -> *mut c_void {
        self.data.offset((self.pitch * y + self.depth * x / 8) as isize)
    }
}
