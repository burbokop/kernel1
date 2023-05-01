
use core::{ffi::c_void, mem::transmute, slice};

#[derive(Debug)]
pub struct FrameBuffer {
    data: *mut c_void,
    pitch: usize,      // len in bytes of one line
    depth: usize,      // bits per pixel
    w: usize,          // pixels in row
    h: usize,          // pixels in column
}

impl FrameBuffer {
    #[inline] pub fn data(&self) -> *mut c_void { self.data }
    #[inline] pub fn pitch(&self) -> usize { self.pitch }
    #[inline] pub fn depth(&self) -> usize { self.depth }
    #[inline] pub fn w(&self) -> usize { self.w }
    #[inline] pub fn h(&self) -> usize { self.h }

    /// unsafe because requere "VGA 320x200 256 color" mode to be enabled in BIOS
    #[inline] pub unsafe fn vga320x200_256color() -> Self {
        Self {
            data: transmute(0xA0000 as usize),
            pitch: 320,
            depth: 8,
            w: 320,
            h: 200,
        }
    }

    #[inline] pub unsafe fn pixel_addr(&mut self, x: usize, y: usize) -> *mut c_void {
        self.data.offset((self.pitch * y + self.depth * x / 8) as isize)
    }

    #[inline] pub unsafe fn as_ref_mut<T>(&mut self) -> &mut [T] {
        slice::from_raw_parts_mut(self.data as *mut T, self.depth* self.pitch as usize * self.h as usize / 8)
    }
}
