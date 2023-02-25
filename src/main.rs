use proforma::primitives::color::Color;
use proforma::window::Window;

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

struct State {
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
    pub divisions: u32,
}

fn build_ui(ui: &mut imgui::Ui, state: &mut State) {
    ui.window("ProForma")
        .size([500.0, 500.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Ellipsoid control");
            ui.separator();

            ui.slider("r_x", 0.0, 1.0, &mut state.rx);
            ui.slider("r_y", 0.0, 1.0, &mut state.ry);
            ui.slider("r_z", 0.0, 1.0, &mut state.rz);

            ui.separator();
            ui.separator();

            ui.text("Render control");
            ui.separator();
            ui.slider("Max render division", 1, 64, &mut state.divisions);
        });
}

fn main() {
    let (mut window, event_loop) = Window::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut last_frame = Instant::now();

    let mut state = State {
        rx: 0.5,
        ry: 0.5,
        rz: 0.5,
        divisions: 16,
    };

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
            window.render(|ui| build_ui(ui, &mut state));
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
