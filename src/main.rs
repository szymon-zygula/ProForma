use proforma::primitives::color::Color;
use proforma::window::Window;

use std::time::Instant;

use glow::HasContext;

const WINDOW_TITLE: &'static str = "ProForma";
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const CLEAR_COLOR: Color = Color {
    r: 0.4,
    g: 0.4,
    b: 0.4,
    a: 1.0,
};

#[derive(Debug)]
struct State {
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
    pub divisions: u32,
}

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330

const vec2 verts[3] = vec2[3](
    vec2(0.5f, 1.0f),
    vec2(0.0f, 0.0f),
    vec2(1.0f, 0.0f)
);

out vec2 vert;
out vec4 color;

void main() {
    vert = verts[gl_VertexID];
    color = vec4(vert, 0.25, 1);
    gl_Position = vec4(vert - 0.5, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330

in vec2 vert;
in vec4 color;

out vec4 frag_color;

void main() {
    frag_color = color;
}
"#;

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

    let mut shaders = [
        (glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE, 0),
        (glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE, 0),
    ];

    let gl = window.gl();

    let vertex_array = unsafe { gl.create_vertex_array() }.unwrap();
    let program = unsafe { gl.create_program() }.unwrap();

    for (kind, source, handle) in &mut shaders {
        unsafe {
            let shader = gl.create_shader(*kind).unwrap();
            gl.shader_source(shader, *source);
            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader) {
                panic!("Error compiling shader: {}", gl.get_shader_info_log(shader));
            }

            gl.attach_shader(program, shader);
            *handle = shader;
        }
    }

    unsafe { gl.link_program(program) };
    if unsafe { !gl.get_program_link_status(program) } {
        panic!("Error linking shader: {}", unsafe {
            gl.get_program_info_log(program)
        });
    }

    for &(_, _, shader) in &shaders {
        unsafe {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }
    }

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
            println!("{:?}", state);
            let gl = window.gl();
            unsafe {
                gl.clear(glow::COLOR_BUFFER_BIT);
                gl.use_program(Some(program));
                gl.bind_vertex_array(Some(vertex_array));
                gl.draw_arrays(glow::TRIANGLES, 0, 3);
            }

            window.render(|ui| build_ui(ui, &mut state));
        }
        glutin::event::Event::WindowEvent {
            event: glutin::event::WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
        glutin::event::Event::LoopDestroyed => unsafe {
            window.gl().delete_program(program);
            window.gl().delete_vertex_array(vertex_array);
        },
        event => {
            window.handle_event(event);
        }
    });
}
