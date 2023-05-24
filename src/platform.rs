use core::{time::Duration, cell::RefCell};

use alloc::rc::Rc;
use slint::platform::software_renderer;

use crate::{hw::{timer_freq, timer_tick}, fb::FrameBuffer};


pub trait Surface {
    type Pixel: software_renderer::TargetPixel;

    fn fb(&self) -> &FrameBuffer;
    fn fb_mut(&mut self) -> &mut FrameBuffer;
    fn flush(&mut self);
}

pub struct Platform<S: Surface> {
    window: Rc<software_renderer::MinimalSoftwareWindow>,
    surface: RefCell<S>,
    timer_freq: f64,
    timer_start: f64,
}

impl<S: Surface> Platform<S> {
    pub fn new(surface: S) -> Self {
        Self {
            window: software_renderer::MinimalSoftwareWindow::new(
                software_renderer::RepaintBufferType::ReusedBuffer,
            ),
            surface: RefCell::new(surface),
            timer_freq: timer_freq() as f64,
            timer_start: timer_tick() as f64,
        }
    }
}

impl<S: Surface> slint::platform::Platform for Platform<S> {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }

    fn duration_since_start(&self) -> Duration {
        Duration::from_secs_f64((timer_tick() as f64 - self.timer_start) / self.timer_freq)
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        {
            let surface = self.surface.borrow();

            self.window.set_size(slint::PhysicalSize::new(
                surface.fb().w() as u32,
                surface.fb().h() as u32,
            ));
        }

        // TODO disable drawing by events for enabling text mode
        let drawing_enabled = true;
        let should_exit = false;

        while !should_exit {
            if drawing_enabled {
                slint::platform::update_timers_and_animations();
            }

            /* TODO
            while poll_event() {
            }
            */

            if drawing_enabled {

                self.window.draw_if_needed(|renderer| {
                    let mut surface = self.surface.borrow_mut();
                    let fb = surface.fb_mut();
                    let pitch = fb.pitch();
                    renderer.render(fb.as_ref_mut::<S::Pixel>(), pitch);
                    surface.flush();
                });

                //if !self.window.has_active_animations() {
                //    wait_for_input(slint::platform::duration_until_next_timer_update());
                //}
            }

            let tt = timer_tick();
            while timer_tick() - tt < 100_000_000 {

            }
        };
        Ok(())
    }
}
