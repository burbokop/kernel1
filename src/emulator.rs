use core::time::Duration;

use alloc::{
    rc::Rc,
    vec::Vec
};

use mini_winclient::{winclient, socket::LOCALHOST, time::Point};
use slint::platform::software_renderer;

use crate::hw::{
    timer_freq,
    timer_tick
};

pub struct Platform {
    window: Rc<software_renderer::MinimalSoftwareWindow>,
    timer_freq: f64,
    timer_start: f64,
}

impl Default for Platform {
    fn default() -> Self {
        Self {
            window: software_renderer::MinimalSoftwareWindow::new(
                software_renderer::RepaintBufferType::ReusedBuffer,
            ),
            timer_freq: timer_freq() as f64,
            timer_start: timer_tick() as f64,
        }
    }
}
/*
fn sdlsym_to_slint(sym: SDL_Keysym) -> SharedString {

    let c = |c: char| -> SharedString {
        if sym.mod_ == SDL_Keymod::KMOD_LSHIFT as u16 || sym.mod_ == SDL_Keymod::KMOD_RSHIFT as u16 {
            c.to_ascii_uppercase().into()
        } else {
            c.to_ascii_lowercase().into()
        }
    };

    match sym.scancode {
        SDL_Scancode::SDL_SCANCODE_UNKNOWN => todo!(),
        SDL_Scancode::SDL_SCANCODE_A => c('a'),
        SDL_Scancode::SDL_SCANCODE_B => c('b'),
        SDL_Scancode::SDL_SCANCODE_C => c('C'),
        SDL_Scancode::SDL_SCANCODE_D => c('d'),
        SDL_Scancode::SDL_SCANCODE_E => c('e'),
        SDL_Scancode::SDL_SCANCODE_F => c('f'),
        SDL_Scancode::SDL_SCANCODE_G => todo!(),
        SDL_Scancode::SDL_SCANCODE_H => todo!(),
        SDL_Scancode::SDL_SCANCODE_I => todo!(),
        SDL_Scancode::SDL_SCANCODE_J => todo!(),
        SDL_Scancode::SDL_SCANCODE_K => todo!(),
        SDL_Scancode::SDL_SCANCODE_L => todo!(),
        SDL_Scancode::SDL_SCANCODE_M => todo!(),
        SDL_Scancode::SDL_SCANCODE_N => todo!(),
        SDL_Scancode::SDL_SCANCODE_O => todo!(),
        SDL_Scancode::SDL_SCANCODE_P => todo!(),
        SDL_Scancode::SDL_SCANCODE_Q => todo!(),
        SDL_Scancode::SDL_SCANCODE_R => todo!(),
        SDL_Scancode::SDL_SCANCODE_S => todo!(),
        SDL_Scancode::SDL_SCANCODE_T => todo!(),
        SDL_Scancode::SDL_SCANCODE_U => todo!(),
        SDL_Scancode::SDL_SCANCODE_V => todo!(),
        SDL_Scancode::SDL_SCANCODE_W => todo!(),
        SDL_Scancode::SDL_SCANCODE_X => todo!(),
        SDL_Scancode::SDL_SCANCODE_Y => todo!(),
        SDL_Scancode::SDL_SCANCODE_Z => todo!(),
        SDL_Scancode::SDL_SCANCODE_1 => todo!(),
        SDL_Scancode::SDL_SCANCODE_2 => todo!(),
        SDL_Scancode::SDL_SCANCODE_3 => todo!(),
        SDL_Scancode::SDL_SCANCODE_4 => todo!(),
        SDL_Scancode::SDL_SCANCODE_5 => todo!(),
        SDL_Scancode::SDL_SCANCODE_6 => todo!(),
        SDL_Scancode::SDL_SCANCODE_7 => todo!(),
        SDL_Scancode::SDL_SCANCODE_8 => todo!(),
        SDL_Scancode::SDL_SCANCODE_9 => todo!(),
        SDL_Scancode::SDL_SCANCODE_0 => todo!(),
        SDL_Scancode::SDL_SCANCODE_RETURN => todo!(),
        SDL_Scancode::SDL_SCANCODE_ESCAPE => todo!(),
        SDL_Scancode::SDL_SCANCODE_BACKSPACE => todo!(),
        SDL_Scancode::SDL_SCANCODE_TAB => todo!(),
        SDL_Scancode::SDL_SCANCODE_SPACE => todo!(),
        SDL_Scancode::SDL_SCANCODE_MINUS => todo!(),
        SDL_Scancode::SDL_SCANCODE_EQUALS => todo!(),
        SDL_Scancode::SDL_SCANCODE_LEFTBRACKET => todo!(),
        SDL_Scancode::SDL_SCANCODE_RIGHTBRACKET => todo!(),
        SDL_Scancode::SDL_SCANCODE_BACKSLASH => todo!(),
        SDL_Scancode::SDL_SCANCODE_NONUSHASH => todo!(),
        SDL_Scancode::SDL_SCANCODE_SEMICOLON => todo!(),
        SDL_Scancode::SDL_SCANCODE_APOSTROPHE => todo!(),
        SDL_Scancode::SDL_SCANCODE_GRAVE => todo!(),
        SDL_Scancode::SDL_SCANCODE_COMMA => todo!(),
        SDL_Scancode::SDL_SCANCODE_PERIOD => todo!(),
        SDL_Scancode::SDL_SCANCODE_SLASH => todo!(),
        SDL_Scancode::SDL_SCANCODE_CAPSLOCK => todo!(),
        SDL_Scancode::SDL_SCANCODE_F1 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F2 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F3 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F4 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F5 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F6 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F7 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F8 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F9 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F10 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F11 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F12 => todo!(),
        SDL_Scancode::SDL_SCANCODE_PRINTSCREEN => todo!(),
        SDL_Scancode::SDL_SCANCODE_SCROLLLOCK => todo!(),
        SDL_Scancode::SDL_SCANCODE_PAUSE => todo!(),
        SDL_Scancode::SDL_SCANCODE_INSERT => todo!(),
        SDL_Scancode::SDL_SCANCODE_HOME => todo!(),
        SDL_Scancode::SDL_SCANCODE_PAGEUP => todo!(),
        SDL_Scancode::SDL_SCANCODE_DELETE => todo!(),
        SDL_Scancode::SDL_SCANCODE_END => todo!(),
        SDL_Scancode::SDL_SCANCODE_PAGEDOWN => todo!(),
        SDL_Scancode::SDL_SCANCODE_RIGHT => todo!(),
        SDL_Scancode::SDL_SCANCODE_LEFT => todo!(),
        SDL_Scancode::SDL_SCANCODE_DOWN => todo!(),
        SDL_Scancode::SDL_SCANCODE_UP => todo!(),
        SDL_Scancode::SDL_SCANCODE_NUMLOCKCLEAR => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_DIVIDE => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_MULTIPLY => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_MINUS => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_PLUS => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_ENTER => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_1 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_2 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_3 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_4 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_5 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_6 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_7 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_8 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_9 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_0 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_PERIOD => todo!(),
        SDL_Scancode::SDL_SCANCODE_NONUSBACKSLASH => todo!(),
        SDL_Scancode::SDL_SCANCODE_APPLICATION => todo!(),
        SDL_Scancode::SDL_SCANCODE_POWER => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_EQUALS => todo!(),
        SDL_Scancode::SDL_SCANCODE_F13 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F14 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F15 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F16 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F17 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F18 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F19 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F20 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F21 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F22 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F23 => todo!(),
        SDL_Scancode::SDL_SCANCODE_F24 => todo!(),
        SDL_Scancode::SDL_SCANCODE_EXECUTE => todo!(),
        SDL_Scancode::SDL_SCANCODE_HELP => todo!(),
        SDL_Scancode::SDL_SCANCODE_MENU => todo!(),
        SDL_Scancode::SDL_SCANCODE_SELECT => todo!(),
        SDL_Scancode::SDL_SCANCODE_STOP => todo!(),
        SDL_Scancode::SDL_SCANCODE_AGAIN => todo!(),
        SDL_Scancode::SDL_SCANCODE_UNDO => todo!(),
        SDL_Scancode::SDL_SCANCODE_CUT => todo!(),
        SDL_Scancode::SDL_SCANCODE_COPY => todo!(),
        SDL_Scancode::SDL_SCANCODE_PASTE => todo!(),
        SDL_Scancode::SDL_SCANCODE_FIND => todo!(),
        SDL_Scancode::SDL_SCANCODE_MUTE => todo!(),
        SDL_Scancode::SDL_SCANCODE_VOLUMEUP => todo!(),
        SDL_Scancode::SDL_SCANCODE_VOLUMEDOWN => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_COMMA => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_EQUALSAS400 => todo!(),
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL1 => todo!(),
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL2 => todo!(),
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL3 => todo!(),
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL4 => todo!(),
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL5 => todo!(),
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL6 => todo!(),
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL7 => todo!(),
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL8 => todo!(),
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL9 => todo!(),
        SDL_Scancode::SDL_SCANCODE_LANG1 => todo!(),
        SDL_Scancode::SDL_SCANCODE_LANG2 => todo!(),
        SDL_Scancode::SDL_SCANCODE_LANG3 => todo!(),
        SDL_Scancode::SDL_SCANCODE_LANG4 => todo!(),
        SDL_Scancode::SDL_SCANCODE_LANG5 => todo!(),
        SDL_Scancode::SDL_SCANCODE_LANG6 => todo!(),
        SDL_Scancode::SDL_SCANCODE_LANG7 => todo!(),
        SDL_Scancode::SDL_SCANCODE_LANG8 => todo!(),
        SDL_Scancode::SDL_SCANCODE_LANG9 => todo!(),
        SDL_Scancode::SDL_SCANCODE_ALTERASE => todo!(),
        SDL_Scancode::SDL_SCANCODE_SYSREQ => todo!(),
        SDL_Scancode::SDL_SCANCODE_CANCEL => todo!(),
        SDL_Scancode::SDL_SCANCODE_CLEAR => todo!(),
        SDL_Scancode::SDL_SCANCODE_PRIOR => todo!(),
        SDL_Scancode::SDL_SCANCODE_RETURN2 => todo!(),
        SDL_Scancode::SDL_SCANCODE_SEPARATOR => todo!(),
        SDL_Scancode::SDL_SCANCODE_OUT => todo!(),
        SDL_Scancode::SDL_SCANCODE_OPER => todo!(),
        SDL_Scancode::SDL_SCANCODE_CLEARAGAIN => todo!(),
        SDL_Scancode::SDL_SCANCODE_CRSEL => todo!(),
        SDL_Scancode::SDL_SCANCODE_EXSEL => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_00 => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_000 => todo!(),
        SDL_Scancode::SDL_SCANCODE_THOUSANDSSEPARATOR => todo!(),
        SDL_Scancode::SDL_SCANCODE_DECIMALSEPARATOR => todo!(),
        SDL_Scancode::SDL_SCANCODE_CURRENCYUNIT => todo!(),
        SDL_Scancode::SDL_SCANCODE_CURRENCYSUBUNIT => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_LEFTPAREN => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_RIGHTPAREN => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_LEFTBRACE => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_RIGHTBRACE => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_TAB => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_BACKSPACE => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_A => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_B => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_C => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_D => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_E => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_F => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_XOR => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_POWER => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_PERCENT => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_LESS => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_GREATER => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_AMPERSAND => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_DBLAMPERSAND => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_VERTICALBAR => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_DBLVERTICALBAR => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_COLON => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_HASH => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_SPACE => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_AT => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_EXCLAM => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_MEMSTORE => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_MEMRECALL => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_MEMCLEAR => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_MEMADD => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_MEMSUBTRACT => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_MEMMULTIPLY => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_MEMDIVIDE => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_PLUSMINUS => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_CLEAR => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_CLEARENTRY => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_BINARY => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_OCTAL => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_DECIMAL => todo!(),
        SDL_Scancode::SDL_SCANCODE_KP_HEXADECIMAL => todo!(),
        SDL_Scancode::SDL_SCANCODE_LCTRL => todo!(),
        SDL_Scancode::SDL_SCANCODE_LSHIFT => Key::Shift.into(),
        SDL_Scancode::SDL_SCANCODE_LALT => todo!(),
        SDL_Scancode::SDL_SCANCODE_LGUI => todo!(),
        SDL_Scancode::SDL_SCANCODE_RCTRL => todo!(),
        SDL_Scancode::SDL_SCANCODE_RSHIFT => todo!(),
        SDL_Scancode::SDL_SCANCODE_RALT => todo!(),
        SDL_Scancode::SDL_SCANCODE_RGUI => todo!(),
        SDL_Scancode::SDL_SCANCODE_MODE => todo!(),
        SDL_Scancode::SDL_SCANCODE_AUDIONEXT => todo!(),
        SDL_Scancode::SDL_SCANCODE_AUDIOPREV => todo!(),
        SDL_Scancode::SDL_SCANCODE_AUDIOSTOP => todo!(),
        SDL_Scancode::SDL_SCANCODE_AUDIOPLAY => todo!(),
        SDL_Scancode::SDL_SCANCODE_AUDIOMUTE => todo!(),
        SDL_Scancode::SDL_SCANCODE_MEDIASELECT => todo!(),
        SDL_Scancode::SDL_SCANCODE_WWW => todo!(),
        SDL_Scancode::SDL_SCANCODE_MAIL => todo!(),
        SDL_Scancode::SDL_SCANCODE_CALCULATOR => todo!(),
        SDL_Scancode::SDL_SCANCODE_COMPUTER => todo!(),
        SDL_Scancode::SDL_SCANCODE_AC_SEARCH => todo!(),
        SDL_Scancode::SDL_SCANCODE_AC_HOME => todo!(),
        SDL_Scancode::SDL_SCANCODE_AC_BACK => todo!(),
        SDL_Scancode::SDL_SCANCODE_AC_FORWARD => todo!(),
        SDL_Scancode::SDL_SCANCODE_AC_STOP => todo!(),
        SDL_Scancode::SDL_SCANCODE_AC_REFRESH => todo!(),
        SDL_Scancode::SDL_SCANCODE_AC_BOOKMARKS => todo!(),
        SDL_Scancode::SDL_SCANCODE_BRIGHTNESSDOWN => todo!(),
        SDL_Scancode::SDL_SCANCODE_BRIGHTNESSUP => todo!(),
        SDL_Scancode::SDL_SCANCODE_DISPLAYSWITCH => todo!(),
        SDL_Scancode::SDL_SCANCODE_KBDILLUMTOGGLE => todo!(),
        SDL_Scancode::SDL_SCANCODE_KBDILLUMDOWN => todo!(),
        SDL_Scancode::SDL_SCANCODE_KBDILLUMUP => todo!(),
        SDL_Scancode::SDL_SCANCODE_EJECT => todo!(),
        SDL_Scancode::SDL_SCANCODE_SLEEP => todo!(),
        SDL_Scancode::SDL_SCANCODE_APP1 => todo!(),
        SDL_Scancode::SDL_SCANCODE_APP2 => todo!(),
        SDL_Scancode::SDL_SCANCODE_AUDIOREWIND => todo!(),
        SDL_Scancode::SDL_SCANCODE_AUDIOFASTFORWARD => todo!(),
        SDL_Scancode::SDL_NUM_SCANCODES => todo!(),
    }
}

fn mouse_btn(button: u8) -> PointerEventButton {
    match button as u32 {
        SDL_BUTTON_LEFT => PointerEventButton::Left,
        SDL_BUTTON_MIDDLE => PointerEventButton::Middle,
        SDL_BUTTON_RIGHT => PointerEventButton::Right,
        SDL_BUTTON_X1 => todo!(), /* add warning to stderr */
        SDL_BUTTON_X2 => todo!(), /* add warning to stderr */
        _ => PointerEventButton::Other,
    }
}
 */
#[repr(transparent)]
#[derive(Clone, Copy)]
struct SlintBltPixel(u32);

impl software_renderer::TargetPixel for SlintBltPixel {
    fn blend(&mut self, color: software_renderer::PremultipliedRgbaColor) {
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
        SlintBltPixel(
            (0xff as u32) << 24 |
            (red as u32) << 16 |
            (green as u32) << 8 |
            (blue as u32) << 0
        )
    }
}

impl slint::platform::Platform for Platform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }

    fn duration_since_start(&self) -> Duration {
        Duration::from_secs_f64((timer_tick() as f64 - self.timer_start) / self.timer_freq)
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        let mut client: winclient::Client<256, 64> = winclient::Client::connect(LOCALHOST, 4321).unwrap();
        client.set_non_blocking_mode(true).unwrap();

        const INITIAL_W: usize = 320;
        const INITIAL_H: usize = 200;

        self.window.set_size(slint::PhysicalSize::new(
            INITIAL_W as u32,
            INITIAL_H as u32,
        ));

        let window_visible = true;
        let mut should_exit = false;

        while !should_exit {
            if window_visible {
                slint::platform::update_timers_and_animations();
            }

            while let Some(package) = client.read_package().unwrap() {
                match package {
                    winclient::Package::Init(_) => todo!(),
                    winclient::Package::Event(event) => match event {
                        mini_winclient::event::Event::Close => should_exit = true,
                        mini_winclient::event::Event::Resize { w, h } => {
                            self.window.set_size(slint::PhysicalSize::new(
                                w as u32,
                                h as u32,
                            ));
                        },
                    },
                }
            }

            if window_visible {
                let w = self.window.size().width as usize;
                let h = self.window.size().height as usize;

                self.window.draw_if_needed(|renderer| {
                    let mut pix: Vec<SlintBltPixel> = Vec::with_capacity(w * h);
                    unsafe { pix.set_len(w * h); }

                    renderer.render(&mut pix, w);

                    client.present(winclient::Format::ARGB, w as u16, h as u16, &pix).unwrap();
                });

                if !self.window.has_active_animations() {
                    //wait_for_input(slint::platform::duration_until_next_timer_update());
                }
            }
            Point::now().loop_for(Duration::from_millis(16));
        };

        Ok(())
    }
}
