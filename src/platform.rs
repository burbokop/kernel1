use core::{time::Duration, slice, ptr::null, mem::{transmute, MaybeUninit}, cell::{RefCell, Cell}};

use alloc::rc::Rc;
use slint::{platform::{software_renderer, PointerEventButton, Key}, SharedString, LogicalPosition};

use crate::{cstd::Stdout, hw::{timer_freq, timer_tick}};

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
        let (mut surface, window) = unsafe {
            SDL_Init(SDL_INIT_EVERYTHING);

            let window = SDL_CreateWindow(
                "Emulator 800x600".as_ptr() as *const i8,
                SDL_WINDOWPOS_CENTERED_MASK as i32,
                SDL_WINDOWPOS_CENTERED_MASK as i32,
                800,
                600,
                SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32
            );
            let surface = SDL_GetWindowSurface(window);

            SDL_FillRect(surface, null(), 0xff006000);
            SDL_UpdateWindowSurface(window);
            (&mut *surface, window)
        };

        self.tell_about_surface_size(surface);

        let mut window_visible = false;
        let mut should_exit = false;

        use core::fmt::Write;
        let mut host_stderr = Stdout::new();

        let mut aaa = 0;
        let mut bbb = 0;

        while !should_exit {
            if window_visible {
                unsafe {
                    let mut w: i32 = 0;
                    let mut h: i32 = 0;
                    writeln!(host_stderr, "SDL_GetWindowSize").ok();
                    SDL_GetWindowSize(window, &mut w, &mut h);
                    aaa += 1;
                    writeln!(host_stderr, "{} ws: {}, {}, ss: {}, {}", aaa, w, h, surface.w, surface.h).ok();

                    if surface.w != w || surface.h != h {
                        self.resize(&mut surface, window);
                    }
                }
                slint::platform::update_timers_and_animations();
            }

            let mut event: SDL_Event = unsafe { MaybeUninit::uninit().assume_init() };
            writeln!(host_stderr, "before event").ok();
            while unsafe { SDL_PollEvent(&mut event as *mut SDL_Event) != 0 } {
                bbb += 1;
                writeln!(host_stderr, "{} event", bbb).ok();

                match unsafe { transmute(event.type_) } {
                    SDL_EventType::SDL_FIRSTEVENT => todo!(),
                    SDL_EventType::SDL_QUIT => if !should_exit { panic!("SDL_QUIT event received but window should not exit") },
                    SDL_EventType::SDL_APP_TERMINATING => todo!(),
                    SDL_EventType::SDL_APP_LOWMEMORY => todo!(),
                    SDL_EventType::SDL_APP_WILLENTERBACKGROUND => todo!(),
                    SDL_EventType::SDL_APP_DIDENTERBACKGROUND => todo!(),
                    SDL_EventType::SDL_APP_WILLENTERFOREGROUND => todo!(),
                    SDL_EventType::SDL_APP_DIDENTERFOREGROUND => todo!(),
                    SDL_EventType::SDL_DISPLAYEVENT => todo!(),
                    SDL_EventType::SDL_WINDOWEVENT => {
                        match unsafe { transmute(event.window.event as u32) } {
                            SDL_WindowEventID::SDL_WINDOWEVENT_NONE => todo!(),
                            SDL_WindowEventID::SDL_WINDOWEVENT_SHOWN => window_visible = true,
                            SDL_WindowEventID::SDL_WINDOWEVENT_HIDDEN => todo!(),
                            SDL_WindowEventID::SDL_WINDOWEVENT_EXPOSED => {},
                            SDL_WindowEventID::SDL_WINDOWEVENT_MOVED => {},
                            SDL_WindowEventID::SDL_WINDOWEVENT_RESIZED => unsafe { self.resize(&mut surface, window) },
                            SDL_WindowEventID::SDL_WINDOWEVENT_SIZE_CHANGED => unsafe { self.resize(&mut surface, window) },
                            SDL_WindowEventID::SDL_WINDOWEVENT_MINIMIZED => unsafe { self.resize(&mut surface, window) },
                            SDL_WindowEventID::SDL_WINDOWEVENT_MAXIMIZED => unsafe { self.resize(&mut surface, window) },
                            SDL_WindowEventID::SDL_WINDOWEVENT_RESTORED => unsafe { self.resize(&mut surface, window) },
                            SDL_WindowEventID::SDL_WINDOWEVENT_ENTER => {},
                            SDL_WindowEventID::SDL_WINDOWEVENT_LEAVE => self.window.dispatch_event(slint::platform::WindowEvent::PointerExited),
                            SDL_WindowEventID::SDL_WINDOWEVENT_FOCUS_GAINED => {},
                            SDL_WindowEventID::SDL_WINDOWEVENT_FOCUS_LOST => {},
                            SDL_WindowEventID::SDL_WINDOWEVENT_CLOSE => should_exit = true,
                            SDL_WindowEventID::SDL_WINDOWEVENT_TAKE_FOCUS => todo!(),
                            SDL_WindowEventID::SDL_WINDOWEVENT_HIT_TEST => todo!(),
                        }
                    },
                    SDL_EventType::SDL_KEYDOWN => unsafe {
                        self.window.dispatch_event(slint::platform::WindowEvent::KeyPressed {
                            text: sdlsym_to_slint(event.key.keysym)
                        })
                    },
                    SDL_EventType::SDL_KEYUP => unsafe {
                        self.window.dispatch_event(slint::platform::WindowEvent::KeyReleased {
                            text: sdlsym_to_slint(event.key.keysym)
                        })
                    },
                    SDL_EventType::SDL_TEXTEDITING => {},
                    SDL_EventType::SDL_TEXTINPUT => {},
                    SDL_EventType::SDL_MOUSEMOTION => unsafe {
                        self.window.dispatch_event(slint::platform::WindowEvent::PointerMoved {
                            position: LogicalPosition::new(event.motion.x as f32, event.motion.y as f32)
                        })
                    },
                    SDL_EventType::SDL_MOUSEBUTTONDOWN => unsafe {
                        self.window.dispatch_event(slint::platform::WindowEvent::PointerPressed {
                            position: LogicalPosition::new(event.button.x as f32, event.button.y as f32),
                            button: mouse_btn(event.button.button),
                        })
                    },
                    SDL_EventType::SDL_MOUSEBUTTONUP => unsafe {
                        self.window.dispatch_event(slint::platform::WindowEvent::PointerReleased {
                            position: LogicalPosition::new(event.button.x as f32, event.button.y as f32),
                            button: mouse_btn(event.button.button),
                        })
                    },
                    SDL_EventType::SDL_MOUSEWHEEL => unsafe {
                        self.window.dispatch_event(slint::platform::WindowEvent::PointerScrolled {
                            position: LogicalPosition::new(event.wheel.x as f32, event.wheel.y as f32),
                            delta_x: event.wheel.x as f32,
                            delta_y: event.wheel.y as f32,
                        })
                    },
                    SDL_EventType::SDL_JOYAXISMOTION => todo!(),
                    SDL_EventType::SDL_JOYBALLMOTION => todo!(),
                    SDL_EventType::SDL_JOYHATMOTION => todo!(),
                    SDL_EventType::SDL_JOYBUTTONDOWN => todo!(),
                    SDL_EventType::SDL_JOYBUTTONUP => todo!(),
                    SDL_EventType::SDL_JOYDEVICEADDED => todo!(),
                    SDL_EventType::SDL_JOYDEVICEREMOVED => todo!(),
                    SDL_EventType::SDL_CONTROLLERAXISMOTION => todo!(),
                    SDL_EventType::SDL_CONTROLLERBUTTONDOWN => todo!(),
                    SDL_EventType::SDL_CONTROLLERBUTTONUP => todo!(),
                    SDL_EventType::SDL_CONTROLLERDEVICEADDED => todo!(),
                    SDL_EventType::SDL_CONTROLLERDEVICEREMOVED => todo!(),
                    SDL_EventType::SDL_CONTROLLERDEVICEREMAPPED => todo!(),
                    SDL_EventType::SDL_CONTROLLERTOUCHPADDOWN => todo!(),
                    SDL_EventType::SDL_CONTROLLERTOUCHPADMOTION => todo!(),
                    SDL_EventType::SDL_CONTROLLERTOUCHPADUP => todo!(),
                    SDL_EventType::SDL_CONTROLLERSENSORUPDATE => todo!(),

                    SDL_EventType::SDL_FINGERDOWN => todo!(),
                    SDL_EventType::SDL_FINGERUP => todo!(),
                    SDL_EventType::SDL_FINGERMOTION => todo!(),
                    SDL_EventType::SDL_DOLLARGESTURE => todo!(),
                    SDL_EventType::SDL_DOLLARRECORD => todo!(),
                    SDL_EventType::SDL_MULTIGESTURE => todo!(),

                    SDL_EventType::SDL_CLIPBOARDUPDATE => todo!(),
                    SDL_EventType::SDL_DROPFILE => todo!(),
                    SDL_EventType::SDL_DROPTEXT => todo!(),
                    SDL_EventType::SDL_DROPBEGIN => todo!(),
                    SDL_EventType::SDL_DROPCOMPLETE => todo!(),

                    SDL_EventType::SDL_AUDIODEVICEADDED => {},
                    SDL_EventType::SDL_AUDIODEVICEREMOVED => {},

                    SDL_EventType::SDL_RENDER_TARGETS_RESET => todo!(),
                    SDL_EventType::SDL_RENDER_DEVICE_RESET => todo!(),

                    SDL_EventType::SDL_USEREVENT => todo!(),
                    SDL_EventType::SDL_LASTEVENT => todo!(),

                    SDL_EventType::SDL_LOCALECHANGED => todo!(),
                    SDL_EventType::SDL_SYSWMEVENT => todo!(),
                    SDL_EventType::SDL_KEYMAPCHANGED => todo!(),
                    SDL_EventType::SDL_SENSORUPDATE => todo!(),
                }
            }

            if window_visible {
                self.window.draw_if_needed(|renderer| {
                    let fb = unsafe {
                        slice::from_raw_parts_mut(surface.pixels as *mut SlintBltPixel, surface.w as usize * surface.h as usize)
                    };
                    //writeln!(host_stderr, "renderer: {:?}", renderer.w).ok();

                    renderer.render(fb, surface.w as usize);
                    unsafe {
                        SDL_UpdateWindowSurface(window);
                    }
                });

                //if !self.window.has_active_animations() {
                //    wait_for_input(slint::platform::duration_until_next_timer_update());
                //}
            }
        };
        unsafe {
            SDL_FreeSurface(surface);
            SDL_DestroyWindow(window);
        }
        Ok(())
    }
}
