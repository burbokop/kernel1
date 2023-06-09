use core::{mem::{transmute, size_of}};

use alloc::vec::Vec;
use slint::platform::software_renderer::{TargetPixel, PremultipliedRgbaColor};

use crate::fb::{FrameBuffer, BufSizeTooSmallError};

static palette: [u32; 256] = [
    0xff000000,
    0xff0002aa,
    0xff14aa00,
    0xff00aaaa,
    0xffaa0003,
    0xffaa00aa,
    0xffaa5500,
    0xffaaaaaa,
    0xff555555,
    0xff5555ff,
    0xff55ff55,
    0xff55ffff,
    0xffff5555,
    0xfffd55ff,
    0xffffff55,
    0xffffffff,
    0xff000000,
    0xff101010,
    0xff202020,
    0xff353535,
    0xff454545,
    0xff555555,
    0xff656565,
    0xff757575,
    0xff8a8a8a,
    0xff9a9a9a,
    0xffaaaaaa,
    0xffbababa,
    0xffcacaca,
    0xffdfdfdf,
    0xffefefef,
    0xffffffff,
    0xff0004ff,
    0xff4104ff,
    0xff8203ff,
    0xffbe02ff,
    0xfffd00ff,
    0xfffe00be,
    0xffff0082,
    0xffff0041,
    0xffff0008,
    0xffff4105,
    0xffff8200,
    0xffffbe00,
    0xffffff00,
    0xffbeff00,
    0xff82ff00,
    0xff41ff01,
    0xff24ff00,
    0xff22ff42,
    0xff1dff82,
    0xff12ffbe,
    0xff00ffff,
    0xff00beff,
    0xff0182ff,
    0xff0041ff,
    0xff8282ff,
    0xff9e82ff,
    0xffbe82ff,
    0xffdf82ff,
    0xfffd82ff,
    0xfffe82df,
    0xffff82be,
    0xffff829e,
    0xffff8282,
    0xffff9e82,
    0xffffbe82,
    0xffffdf82,
    0xffffff82,
    0xffdfff82,
    0xffbeff82,
    0xff9eff82,
    0xff82ff82,
    0xff82ff9e,
    0xff82ffbe,
    0xff82ffdf,
    0xff82ffff,
    0xff82dfff,
    0xff82beff,
    0xff829eff,
    0xffbabaff,
    0xffcabaff,
    0xffdfbaff,
    0xffefbaff,
    0xfffebaff,
    0xfffebaef,
    0xffffbadf,
    0xffffbaca,
    0xffffbaba,
    0xffffcaba,
    0xffffdfba,
    0xffffefba,
    0xffffffba,
    0xffefffba,
    0xffdfffba,
    0xffcaffbb,
    0xffbaffba,
    0xffbaffca,
    0xffbaffdf,
    0xffbaffef,
    0xffbaffff,
    0xffbaefff,
    0xffbadfff,
    0xffbacaff,
    0xff010171,
    0xff1c0171,
    0xff390171,
    0xff550071,
    0xff710071,
    0xff710055,
    0xff710039,
    0xff71001c,
    0xff710001,
    0xff711c01,
    0xff713900,
    0xff715500,
    0xff717100,
    0xff557100,
    0xff397100,
    0xff1c7100,
    0xff097100,
    0xff09711c,
    0xff067139,
    0xff037155,
    0xff007171,
    0xff005571,
    0xff003971,
    0xff001c71,
    0xff393971,
    0xff453971,
    0xff553971,
    0xff613971,
    0xff713971,
    0xff713961,
    0xff713955,
    0xff713945,
    0xff713939,
    0xff714539,
    0xff715539,
    0xff716139,
    0xff717139,
    0xff617139,
    0xff557139,
    0xff45713a,
    0xff397139,
    0xff397145,
    0xff397155,
    0xff397161,
    0xff397171,
    0xff396171,
    0xff395571,
    0xff394572,
    0xff515171,
    0xff595171,
    0xff615171,
    0xff695171,
    0xff715171,
    0xff715169,
    0xff715161,
    0xff715159,
    0xff715151,
    0xff715951,
    0xff716151,
    0xff716951,
    0xff717151,
    0xff697151,
    0xff617151,
    0xff597151,
    0xff517151,
    0xff51715a,
    0xff517161,
    0xff517169,
    0xff517171,
    0xff516971,
    0xff516171,
    0xff515971,
    0xff000042,
    0xff110041,
    0xff200041,
    0xff310041,
    0xff410041,
    0xff410032,
    0xff410020,
    0xff410010,
    0xff410000,
    0xff411000,
    0xff412000,
    0xff413100,
    0xff414100,
    0xff314100,
    0xff204100,
    0xff104100,
    0xff034100,
    0xff034110,
    0xff024120,
    0xff014131,
    0xff004141,
    0xff003141,
    0xff002041,
    0xff001041,
    0xff202041,
    0xff282041,
    0xff312041,
    0xff392041,
    0xff412041,
    0xff412039,
    0xff412031,
    0xff412028,
    0xff412020,
    0xff412820,
    0xff413120,
    0xff413921,
    0xff414120,
    0xff394120,
    0xff314120,
    0xff284120,
    0xff204120,
    0xff204128,
    0xff204131,
    0xff204139,
    0xff204141,
    0xff203941,
    0xff203141,
    0xff202841,
    0xff2d2d41,
    0xff312d41,
    0xff352d41,
    0xff3d2d41,
    0xff412d41,
    0xff412d3d,
    0xff412d35,
    0xff412d31,
    0xff412d2d,
    0xff41312d,
    0xff41352d,
    0xff413d2d,
    0xff41412d,
    0xff3d412d,
    0xff35412d,
    0xff31412d,
    0xff2d412d,
    0xff2d4131,
    0xff2d4135,
    0xff2d413d,
    0xff2d4141,
    0xff2d3d41,
    0xff2d3541,
    0xff2d3141,
    0xff000000,
    0xff000000,
    0xff000000,
    0xff000000,
    0xff000000,
    0xff000000,
    0xff000000,
    0xff000000,
];



#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Pixel(u32);

impl TargetPixel for Pixel {
    fn blend(&mut self, color: PremultipliedRgbaColor) {
        let a = (u8::MAX - color.alpha) as u16;

        let alpha = (self.0 >> 24) as u8;
        let red = (self.0 >> 16) as u8;
        let green = (self.0 >> 8) as u8;
        let blue = (self.0 >> 0) as u8;

        let alpha = (alpha as u16 * a / 255) as u8 + color.alpha;
        let red = (red as u16 * a / 255) as u8 + color.red;
        let green = (green as u16 * a / 255) as u8 + color.green;
        let blue = (blue as u16 * a / 255) as u8 + color.blue;
        self.0 =
            (alpha as u32) << 24 |
            (red as u32) << 16 |
            (green as u32) << 8 |
            (blue as u32) << 0;
    }

    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Pixel(
            (0xff as u32) << 24 |
            (red as u32) << 16 |
            (green as u32) << 8 |
            (blue as u32) << 0
        )
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct VgaPixel(u8);

impl VgaPixel {
    fn vga256c_to_argb(src: u8) -> u32 {
        palette[src as usize]
    }

    fn avr_abs_diff(x: u32, y: u32) -> u8 {
        let a = ((x >> 24) as u8).abs_diff((y >> 24) as u8);
        let r = ((x >> 16) as u8).abs_diff((y >> 16) as u8);
        let g = ((x >> 08) as u8).abs_diff((y >> 08) as u8);
        let b = ((x >> 00) as u8).abs_diff((y >> 00) as u8);

        ((a as u32 + r as u32 + g as u32 + b as u32) / 4) as u8
    }

    fn argb_to_vga256c(src: Pixel) -> VgaPixel {
        VgaPixel(palette
            .into_iter()
            .map(|x| Self::avr_abs_diff(x, src.0))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0 as u8)
    }
}


pub struct Surface {
    primary_fb: FrameBuffer,
    secondary_fb: FrameBuffer,
    buf: Vec<u8>,
}

impl Surface {
    pub unsafe fn new() -> Self {
        Self::from_fb(FrameBuffer::vga320x200_256color())
    }

    pub fn from_fb(fb: FrameBuffer) -> Self {
        let mut b: Vec<u8> = Vec::with_capacity(fb.w() * fb.h() * 32 / 8);
        unsafe { b.set_len(b.capacity()); }

        Self {
            primary_fb: FrameBuffer::from_raw_slice(&mut b, fb.w(), fb.h(), 32).unwrap(),
            secondary_fb: fb,
            buf: b,
        }
    }
}

impl crate::platform::Surface for Surface {
    type Pixel = Pixel;

    fn fb(&self) -> &crate::fb::FrameBuffer {
        &self.primary_fb
    }

    fn fb_mut(&mut self) -> &mut crate::fb::FrameBuffer {
        &mut self.primary_fb
    }

    fn flush(&mut self) {
        let from: &mut[Pixel] = self.primary_fb.as_ref_mut();
        let to: &mut[VgaPixel] = self.secondary_fb.as_ref_mut();
        assert_eq!(from.len(), to.len());

        for (f, t) in from.iter().zip(to.iter_mut()) {
            *t = VgaPixel::argb_to_vga256c(*f)
        }
    }
}
