mod forms;
mod math;
mod primitives;
mod window;

use glow::HasContext;
use primitives::color::Color;
use window::Window;

use std::time::Instant;

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
    let (mut window, event_loop) = Window::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut last_frame = Instant::now();

    window.set_clear_color(CLEAR_COLOR);

    event_loop.run(move |event, _, control_flow| match event {
        glutin::event::Event::NewEvents(_) => {
            let now = Instant::now();
            let duration = now.duration_since(last_frame);
            window.update_delta_time(duration);
            last_frame = now;
        }
        glutin::event::Event::MainEventsCleared => {
            window.request_redraw();
        }
        glutin::event::Event::RedrawRequested(_) => {
            window.render(|ui| {
                ui.show_demo_window(&mut true);
            });
        }
        glutin::event::Event::WindowEvent {
            event: glutin::event::WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
        event => {
            window.handle_event(event);
        }
    });
}
