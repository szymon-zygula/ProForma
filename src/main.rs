mod forms;
mod math;
mod primitives;
mod window;

use primitives::color::Color;
use window::Window;

const WINDOW_TITLE: &'static str = "ProForma";
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const CLEAR_COLOR: Color = Color {
    r: 0.4,
    g: 0.4,
    b: 0.4,
    a: 1.0,
};

fn main() {
    let mut window = Window::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);

    window.set_clear_color(&CLEAR_COLOR);

    'render_loop: loop {
        for event in window.events() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'render_loop,
                sdl2::event::Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => match win_event {
                    sdl2::event::WindowEvent::Resized(width, height) => {
                        println!("Window resized to {}x{}", width, height)
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        window.refresh();
    }
}
