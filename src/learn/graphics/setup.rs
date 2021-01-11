use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;

use crate::graphics::render::*;

pub fn graphic_setup(window_size: f64) -> (GlutinWindow, Display, Events) {
    let opengl: OpenGL = OpenGL::V3_2;
    let window: GlutinWindow = WindowSettings::new("ft_linear_regression", [window_size, window_size])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .expect("error: can't initialize the GlutinWindow");
    let display: Display = Display::new(opengl, window_size, 20.0);
    let events: Events = Events::new(EventSettings::new());
    return (window, display, events);
}