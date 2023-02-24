use crate::primitives::color::Color;
use glow::*;
use sdl2;

pub struct Window {
    gl: glow::Context,
    gl_context: sdl2::video::GLContext,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);
        gl_attr.set_context_flags().forward_compatible().set();

        let window = video
            .window(title, width, height)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        let gl = unsafe {
            glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
        };

        let event_pump = sdl.event_pump().unwrap();

        return Window {
            gl,
            gl_context,
            window,
            event_pump,
        };
    }

    pub fn set_clear_color(&mut self, color: &Color) {
        unsafe { self.gl.clear_color(color.r, color.g, color.b, color.a) };
    }

    pub fn refresh(&mut self) {
        unsafe {
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }

        self.window.gl_swap_window();
    }

    pub fn events<'a>(&'a mut self) -> sdl2::event::EventPollIterator<'a> {
        self.event_pump.poll_iter()
    }
}
