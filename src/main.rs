use proforma::{
    forms::{ellipsoid::Ellipsoid, implicit::QuadraticForm},
    math::{
        affine::{
            self,
            primitives::{Point, Vector},
        },
        matrix::Matrix,
    },
    primitives::color::Color,
    window::Window,
};

use std::time::Instant;

use glow::HasContext;

const WINDOW_TITLE: &str = "ProForma";
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const SCROLL_MULTIPLIER: f64 = 0.005;
const SCALE_STEP: f32 = 50.0;
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
    pub divs: i32,
    pub max_divs: i32,
    pub light_intensity: f64,
    pub left_mouse_button_down: bool,
    pub right_mouse_button_down: bool,
    pub current_mouse_position: Option<glutin::dpi::PhysicalPosition<f64>>,
    pub previous_mouse_position: Option<glutin::dpi::PhysicalPosition<f64>>,
    pub camera_position: Point,
    pub camera_basis: Matrix<f64, 4, 4>,
    pub scroll_delta: f32,
    pub scale: f32,
    pub resolution: glutin::dpi::PhysicalSize<u32>,
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
uniform vec2 resolution;
uniform float scale;
uniform int divs;

const float near_plane = 0.1;

void main() {
    vec2 coord = vec2(vert.x * resolution.x, vert.y * resolution.y);
    coord = round(coord / divs) * divs / scale;
    float free_term = dot(coord.x * qf[0].xyw + coord.y * qf[1].xyw + qf[3].xyw, vec3(coord.xy, 1));
    float line_term = dot(qf[2].xyw + vec3(qf[0].z, qf[1].z, qf[3].z), vec3(coord.xy, 1));
    float quad_term = qf[2].z;

    float delta = line_term * line_term - 4 * free_term * quad_term;

    if(delta >= 0.0) {
        float sqrt_delta = sqrt(delta);
        float s1 = (-line_term - sqrt_delta) / (2 * quad_term);
        float s2 = (-line_term + sqrt_delta) / (2 * quad_term);

        if(s1 > near_plane && s2 > near_plane) {
            frag_color = vec4(1.0, 1.0, 0.0, 1.0);
        }
        else if (s1 > near_plane || s2 > near_plane) {
            frag_color = vec4(0.7, 0.7, 0.0, 1.0);
        }
        else {
            frag_color = vec4(0.5, 0.5, 0.5, 1.0);
        }
    }
    else {
        frag_color = vec4(0.5, 0.5, 0.5, 1.0);
    }
}
"#;

fn build_ui(ui: &mut imgui::Ui, state: &mut State) {
    ui.window("ProForma")
        .size([500.0, 500.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Ellipsoid control");
            ui.slider("r_x", 0.0, 1.0, &mut state.rx);
            ui.slider("r_y", 0.0, 1.0, &mut state.ry);
            ui.slider("r_z", 0.0, 1.0, &mut state.rz);

            ui.separator();
            ui.text("Render control");
            ui.slider("Max render division", 1, 64, &mut state.max_divs);

            ui.separator();
            ui.text("Light control");
            ui.slider("Intensity", 0.0, 1.0, &mut state.light_intensity);

            ui.separator();
            ui.text("Info");
            ui.text(format!(
                "Camera position (x, y, z): {:.4}, {:.4}, {:.4}",
                state.camera_position.at(0),
                state.camera_position.at(1),
                state.camera_position.at(2)
            ));
            ui.text(format!(
                "View vector (x, y, z): {:.4}, {:.4}, {:.4}",
                state.camera_basis.at(0, 2),
                state.camera_basis.at(1, 2),
                state.camera_basis.at(2, 2)
            ));
            ui.text(format!(
                "Up vector (x, y, z): {:.4}, {:.4}, {:.4}",
                state.camera_basis.at(0, 1),
                state.camera_basis.at(1, 1),
                state.camera_basis.at(2, 1)
            ));
            ui.text(format!("Scale: {}", state.scale / SCALE_STEP / 20.0));
        });
}

fn main() {
    let (mut window, event_loop) = Window::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut last_frame = Instant::now();

    let mut app_state = State {
        rx: 0.5,
        ry: 0.5,
        rz: 0.5,
        divs: 1,
        max_divs: 32,
        light_intensity: 0.5,
        left_mouse_button_down: false,
        right_mouse_button_down: false,
        current_mouse_position: None,
        previous_mouse_position: None,
        scroll_delta: 0.0,
        resolution: glutin::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        camera_position: Point::new(0.0, 0.0, 1.0),
        camera_basis: Matrix::from_data([
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]),
        scale: 1000.0,
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
            gl.shader_source(shader, source);
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
            let ellipsoid = Ellipsoid::with_radii(app_state.rx, app_state.ry, app_state.rz);

            unsafe {
                gl.clear(glow::COLOR_BUFFER_BIT);
                gl.use_program(Some(program));

                let (change_x, change_y) = if !window.imgui_using_mouse() {
                    app_state
                        .previous_mouse_position
                        .zip(app_state.current_mouse_position)
                        .map(|(prev, cur)| {
                            (
                                (prev.x - cur.x) * SCROLL_MULTIPLIER,
                                (prev.y - cur.y) * SCROLL_MULTIPLIER,
                            )
                        })
                        .unwrap_or((0.0, 0.0))
                } else {
                    (0.0, 0.0)
                };

                let mut change = false;

                if app_state.scroll_delta != 0.0 {
                    if app_state.right_mouse_button_down {
                        app_state.camera_position = app_state.camera_position
                            + app_state.camera_basis
                                * Vector::new(0.0, 0.0, app_state.scroll_delta as f64 * 0.1);
                    } else if app_state.left_mouse_button_down {
                        app_state.camera_basis = app_state.camera_basis
                            * affine::transforms::rotate_z(app_state.scroll_delta as f64 * 0.1);
                    } else {
                        app_state.scale += app_state.scroll_delta * SCALE_STEP;

                        if app_state.scale < SCALE_STEP {
                            app_state.scale = SCALE_STEP;
                        }
                    }

                    app_state.scroll_delta = 0.0;
                    change = true;
                }

                if app_state.previous_mouse_position.is_some() {
                    app_state.previous_mouse_position = None;
                }

                let mouse_moved = change_x != 0.0 || change_y != 0.0;

                if app_state.left_mouse_button_down && mouse_moved {
                    app_state.camera_basis = app_state.camera_basis
                        * affine::transforms::rotate_y(change_x)
                        * affine::transforms::rotate_x(change_y);
                    change = true;
                } else if app_state.right_mouse_button_down && mouse_moved {
                    app_state.camera_position = app_state.camera_position
                        + app_state.camera_basis * Vector::new(change_x, -change_y, 0.0);
                    change = true;
                }

                if !change {
                    app_state.divs = std::cmp::max(app_state.divs - 1, 1);
                } else {
                    app_state.divs = app_state.max_divs;
                }

                let transform_matrix = app_state.camera_basis.inverse().unwrap()
                    * affine::transforms::translate(-Vector::to_point(app_state.camera_position));

                let inverse_transform = transform_matrix.inverse().unwrap();
                let quadratic_form_matrix = inverse_transform.transpose()
                    * ellipsoid.quadratic_form_matrix()
                    * inverse_transform;

                let quadratic_form_location = gl.get_uniform_location(program, "qf").unwrap();
                gl.uniform_matrix_4_f32_slice(
                    Some(&quadratic_form_location),
                    true,
                    quadratic_form_matrix.with_type::<f32>().raw(),
                );

                let scale_location = gl.get_uniform_location(program, "scale").unwrap();
                gl.uniform_1_f32(Some(&scale_location), app_state.scale);

                let divisions_location = gl.get_uniform_location(program, "divs").unwrap();
                gl.uniform_1_i32(Some(&divisions_location), app_state.divs);

                let resolution_location = gl.get_uniform_location(program, "resolution").unwrap();
                gl.uniform_2_f32(
                    Some(&resolution_location),
                    app_state.resolution.width as f32,
                    app_state.resolution.height as f32,
                );

                gl.bind_vertex_array(Some(vertex_array));
                gl.draw_arrays(glow::TRIANGLES, 0, 6);
            }

            window.render(|ui| build_ui(ui, &mut app_state));
        }
        Event::WindowEvent {
            event:
                WindowEvent::MouseWheel {
                    delta: glutin::event::MouseScrollDelta::LineDelta(_, delta),
                    ..
                },
            ..
        } => {
            app_state.scroll_delta = delta;
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
            match event {
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { state, button, .. },
                    ..
                } => {
                    use glutin::event::{ElementState, MouseButton};
                    match (state, button) {
                        (ElementState::Pressed, MouseButton::Left) => {
                            app_state.left_mouse_button_down = true
                        }

                        (ElementState::Released, MouseButton::Left) => {
                            app_state.left_mouse_button_down = false
                        }
                        (ElementState::Pressed, MouseButton::Right) => {
                            app_state.right_mouse_button_down = true
                        }
                        (ElementState::Released, MouseButton::Right) => {
                            app_state.right_mouse_button_down = false
                        }
                        _ => {}
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorLeft { .. },
                    ..
                } => {
                    app_state.left_mouse_button_down = false;
                    app_state.right_mouse_button_down = false;
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    app_state.previous_mouse_position = app_state.current_mouse_position;
                    app_state.current_mouse_position = Some(position);
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    app_state.resolution = size;
                }
                _ => {}
            }
            window.handle_event(event);
        }
    });
}
