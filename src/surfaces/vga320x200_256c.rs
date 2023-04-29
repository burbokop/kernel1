use core::mem::transmute;

use slint::platform::software_renderer::{TargetPixel, PremultipliedRgbaColor};

use crate::fb::FrameBuffer;

static palette: [u32; 256] = [
    0x00000000,
    0x00800000,
    0x00008000,
    0x00808000,
    0x00000080,
    0x00800080,
    0x00008080,
    0x00c0c0c0,
    0x00808080,
    0x00ff0000,
    0x0000ff00,
    0x00ffff00,
    0x000000ff,
    0x00ff00ff,
    0x0000ffff,
    0x00ffffff,
    0x00000000,
    0x0000005f,
    0x00000087,
    0x000000af,
    0x000000d7,
    0x000000ff,
    0x00005f00,
    0x00005f5f,
    0x00005f87,
    0x00005faf,
    0x00005fd7,
    0x00005fff,
    0x00008700,
    0x0000875f,
    0x00008787,
    0x000087af,
    0x000087d7,
    0x000087ff,
    0x0000af00,
    0x0000af5f,
    0x0000af87,
    0x0000afaf,
    0x0000afd7,
    0x0000afff,
    0x0000d700,
    0x0000d75f,
    0x0000d787,
    0x0000d7af,
    0x0000d7d7,
    0x0000d7ff,
    0x0000ff00,
    0x0000ff5f,
    0x0000ff87,
    0x0000ffaf,
    0x0000ffd7,
    0x0000ffff,
    0x005f0000,
    0x005f005f,
    0x005f0087,
    0x005f00af,
    0x005f00d7,
    0x005f00ff,
    0x005f5f00,
    0x005f5f5f,
    0x005f5f87,
    0x005f5faf,
    0x005f5fd7,
    0x005f5fff,
    0x005f8700,
    0x005f875f,
    0x005f8787,
    0x005f87af,
    0x005f87d7,
    0x005f87ff,
    0x005faf00,
    0x005faf5f,
    0x005faf87,
    0x005fafaf,
    0x005fafd7,
    0x005fafff,
    0x005fd700,
    0x005fd75f,
    0x005fd787,
    0x005fd7af,
    0x005fd7d7,
    0x005fd7ff,
    0x005fff00,
    0x005fff5f,
    0x005fff87,
    0x005fffaf,
    0x005fffd7,
    0x005fffff,
    0x00870000,
    0x0087005f,
    0x00870087,
    0x008700af,
    0x008700d7,
    0x008700ff,
    0x00875f00,
    0x00875f5f,
    0x00875f87,
    0x00875faf,
    0x00875fd7,
    0x00875fff,
    0x00878700,
    0x0087875f,
    0x00878787,
    0x008787af,
    0x008787d7,
    0x008787ff,
    0x0087af00,
    0x0087af5f,
    0x0087af87,
    0x0087afaf,
    0x0087afd7,
    0x0087afff,
    0x0087d700,
    0x0087d75f,
    0x0087d787,
    0x0087d7af,
    0x0087d7d7,
    0x0087d7ff,
    0x0087ff00,
    0x0087ff5f,
    0x0087ff87,
    0x0087ffaf,
    0x0087ffd7,
    0x0087ffff,
    0x00af0000,
    0x00af005f,
    0x00af0087,
    0x00af00af,
    0x00af00d7,
    0x00af00ff,
    0x00af5f00,
    0x00af5f5f,
    0x00af5f87,
    0x00af5faf,
    0x00af5fd7,
    0x00af5fff,
    0x00af8700,
    0x00af875f,
    0x00af8787,
    0x00af87af,
    0x00af87d7,
    0x00af87ff,
    0x00afaf00,
    0x00afaf5f,
    0x00afaf87,
    0x00afafaf,
    0x00afafd7,
    0x00afafff,
    0x00afd700,
    0x00afd75f,
    0x00afd787,
    0x00afd7af,
    0x00afd7d7,
    0x00afd7ff,
    0x00afff00,
    0x00afff5f,
    0x00afff87,
    0x00afffaf,
    0x00afffd7,
    0x00afffff,
    0x00d70000,
    0x00d7005f,
    0x00d70087,
    0x00d700af,
    0x00d700d7,
    0x00d700ff,
    0x00d75f00,
    0x00d75f5f,
    0x00d75f87,
    0x00d75faf,
    0x00d75fd7,
    0x00d75fff,
    0x00d78700,
    0x00d7875f,
    0x00d78787,
    0x00d787af,
    0x00d787d7,
    0x00d787ff,
    0x00d7af00,
    0x00d7af5f,
    0x00d7af87,
    0x00d7afaf,
    0x00d7afd7,
    0x00d7afff,
    0x00d7d700,
    0x00d7d75f,
    0x00d7d787,
    0x00d7d7af,
    0x00d7d7d7,
    0x00d7d7ff,
    0x00d7ff00,
    0x00d7ff5f,
    0x00d7ff87,
    0x00d7ffaf,
    0x00d7ffd7,
    0x00d7ffff,
    0x00ff0000,
    0x00ff005f,
    0x00ff0087,
    0x00ff00af,
    0x00ff00d7,
    0x00ff00ff,
    0x00ff5f00,
    0x00ff5f5f,
    0x00ff5f87,
    0x00ff5faf,
    0x00ff5fd7,
    0x00ff5fff,
    0x00ff8700,
    0x00ff875f,
    0x00ff8787,
    0x00ff87af,
    0x00ff87d7,
    0x00ff87ff,
    0x00ffaf00,
    0x00ffaf5f,
    0x00ffaf87,
    0x00ffafaf,
    0x00ffafd7,
    0x00ffafff,
    0x00ffd700,
    0x00ffd75f,
    0x00ffd787,
    0x00ffd7af,
    0x00ffd7d7,
    0x00ffd7ff,
    0x00ffff00,
    0x00ffff5f,
    0x00ffff87,
    0x00ffffaf,
    0x00ffffd7,
    0x00ffffff,
    0x00080808,
    0x00121212,
    0x001c1c1c,
    0x00262626,
    0x00303030,
    0x003a3a3a,
    0x00444444,
    0x004e4e4e,
    0x00585858,
    0x00626262,
    0x006c6c6c,
    0x00767676,
    0x00808080,
    0x008a8a8a,
    0x00949494,
    0x009e9e9e,
    0x00a8a8a8,
    0x00b2b2b2,
    0x00bcbcbc,
    0x00c6c6c6,
    0x00d0d0d0,
    0x00dadada,
    0x00e4e4e4,
    0x00eeeeee,
];




#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Pixel(u8);

impl Pixel {
    fn vga256c_to_argb(src: u8) -> u32 {
        palette[src as usize]
    }

    fn argb_to_vga256c(src: u32) -> u8 {
        palette
            .into_iter()
            .map(|x| x.abs_diff(src))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0 as u8
    }
}

impl TargetPixel for Pixel {


    fn blend(&mut self, color: PremultipliedRgbaColor) {
        let a = (u8::MAX - color.alpha) as u16;

        let argb = Self::vga256c_to_argb(self.0);

        let alpha = (argb >> 24) as u8;
        let red = (argb >> 16) as u8;
        let green = (argb >> 8) as u8;
        let blue = (argb >> 0) as u8;

        let alpha = (alpha as u16 * a / 255) as u8 + color.alpha;
        let red = (red as u16 * a / 255) as u8 + color.red;
        let green = (green as u16 * a / 255) as u8 + color.green;
        let blue = (blue as u16 * a / 255) as u8 + color.blue;
        self.0 = Self::argb_to_vga256c(
            (alpha as u32) << 24 |
            (red as u32) << 16 |
            (green as u32) << 8 |
            (blue as u32) << 0);
    }

    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Pixel(Self::argb_to_vga256c(
            (0x00 as u32) << 24 |
            (red as u32) << 16 |
            (green as u32) << 8 |
            (blue as u32) << 0)
        )
    }
}

pub struct Surface(FrameBuffer);

impl Surface {
    pub unsafe fn new() -> Self {
        Self(FrameBuffer::vga320x200_256color())
    }
}

impl crate::platform::Surface for Surface {
    type Pixel = Pixel;

    fn fb(&self) -> &crate::fb::FrameBuffer {
        &self.0
    }

    fn fb_mut(&mut self) -> &mut crate::fb::FrameBuffer {
        &mut self.0
    }
}
