use proforma::{
    forms::{ellipsoid::Ellipsoid, implicit::QuadraticForm},
    primitives::color::Color,
    window::Window,
};

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
    pub light_intensity: f64,
}

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330

const vec2 verts[6] = vec2[6](
    vec2(-1.0f,  1.0f),
    vec2( 1.0f,  1.0f),
    vec2(-1.0f, -1.0f),
    vec2(-1.0f, -1.0f),
    vec2( 1.0f,  1.0f),
    vec2( 1.0f, -1.0f)
);

out vec2 vert;

void main() {
    vert = verts[gl_VertexID];
    gl_Position = vec4(vert, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330

in vec2 vert;

out vec4 frag_color;

uniform mat4 qf;

void main() {
    float free_term = dot(vert.x * qf[0].xyw + vert.y * qf[1].xyw + qf[3].xyw, vec3(vert.xy, 1));
    float line_term = dot(qf[2].xyw + vec3(qf[0].z, qf[1].z, qf[3].z), vec3(vert.xy, 1));
    float quad_term = qf[2].z;

    float delta = line_term * line_term - 4 * free_term * quad_term;

    if(delta >= 0.0) {
        frag_color = vec4(1.0, 1.0, 0.0, 1.0);
    }
    else {
        frag_color = vec4(0.5, 0.5, 0.5, 1.0);
    }
}
"#;

fn build_ui(ui: &mut imgui::Ui, state: &mut State) {
    ui.show_demo_window(&mut true);
    ui.window("ProForma")
        .size([500.0, 500.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Ellipsoid control");

            ui.slider("r_x", 0.0, 1.0, &mut state.rx);
            ui.slider("r_y", 0.0, 1.0, &mut state.ry);
            ui.slider("r_z", 0.0, 1.0, &mut state.rz);

            ui.separator();

            ui.text("Render control");
            ui.slider("Max render division", 1, 64, &mut state.divisions);

            ui.separator();
            ui.text("Light control");
            ui.slider("Intensity", 0.0, 1.0, &mut state.light_intensity);
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
        light_intensity: 0.5,
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

    use glutin::event::{Event, WindowEvent};
    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(_) => {
            let now = Instant::now();
            let duration = now.duration_since(last_frame);
            window.update_delta_time(duration);
            last_frame = now;
        }
        Event::MainEventsCleared => window.request_redraw(),
        Event::RedrawRequested(_) => {
            let gl = window.gl();
            let ellipsoid = Ellipsoid::with_radii(state.rx, state.ry, state.rz);

            unsafe {
                gl.clear(glow::COLOR_BUFFER_BIT);
                gl.use_program(Some(program));

                let quadratic_form_location = gl.get_uniform_location(program, "qf").unwrap();
                gl.uniform_matrix_4_f32_slice(
                    Some(&quadratic_form_location),
                    true,
                    ellipsoid.quadratic_form_matrix().with_type::<f32>().raw(),
                );

                gl.bind_vertex_array(Some(vertex_array));
                gl.draw_arrays(glow::TRIANGLES, 0, 6);
            }

            window.render(|ui| build_ui(ui, &mut state));
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = glutin::event_loop::ControlFlow::Exit,
        Event::LoopDestroyed => unsafe {
            window.gl().delete_program(program);
            window.gl().delete_vertex_array(vertex_array);
        },
        event => {
            window.handle_event(event);
        }
    });
}
