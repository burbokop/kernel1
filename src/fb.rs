
use core::{ffi::c_void, mem::{transmute, size_of}, slice};

#[derive(Debug)]
pub struct FrameBuffer {
    data: *mut c_void,
    pitch: usize,      // len in bytes of one line
    depth: usize,      // bits per pixel
    w: usize,          // pixels in row
    h: usize,          // pixels in column
}

#[derive(Debug)]
pub struct BufSizeTooSmallError;

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

    pub fn from_raw_slice(data: &mut[u8], w: usize, h: usize, depth: usize) -> Result<Self, BufSizeTooSmallError> {
        if data.len() >= w * h * depth / 8 {
            Ok(Self {
                data: data.as_mut_ptr() as *mut c_void,
                pitch: w,
                depth,
                w,
                h
            })
        } else {
            Err(BufSizeTooSmallError)
        }
    }

    #[inline] pub unsafe fn pixel_addr(&mut self, x: usize, y: usize) -> *mut c_void {
        self.data.offset((self.pitch * y + self.depth * x / 8) as isize)
    }

    #[inline] pub fn as_ref_mut<T>(&mut self) -> &mut [T] {
        // unsafe is reduced because all safe constructors do checks
        unsafe {
            slice::from_raw_parts_mut(
                self.data as *mut T,
                self.depth * self.pitch as usize * self.h as usize / 8 / size_of::<T>()
            )
        }
    }
}
