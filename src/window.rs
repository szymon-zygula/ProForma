use crate::primitives::color::Color;
use glow::*;

use glow::HasContext;
use glutin::{event_loop::EventLoop, WindowedContext};
use imgui_glow_renderer;
use imgui_winit_support::WinitPlatform;

pub struct Window {
    window_context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    imgui_renderer: imgui_glow_renderer::AutoRenderer,
    imgui_context: imgui::Context,
    winit_platform: WinitPlatform,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> (Window, EventLoop<()>) {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(width, height));

        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window, &event_loop)
            .unwrap();

        let window_context = unsafe { window.make_current() }.unwrap();

        let mut imgui_context = imgui::Context::create();
        imgui_context.set_ini_filename(None);

        let mut winit_platform = WinitPlatform::init(&mut imgui_context);
        winit_platform.attach_window(
            imgui_context.io_mut(),
            window_context.window(),
            imgui_winit_support::HiDpiMode::Rounded,
        );

        imgui_context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

        let gl = unsafe {
            glow::Context::from_loader_function(|s| window_context.get_proc_address(s).cast())
        };

        let imgui_renderer =
            imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui_context).unwrap();

        (
            Window {
                window_context,
                imgui_renderer,
                winit_platform,
                imgui_context,
            },
            event_loop,
        )
    }

    pub unsafe fn gl(&mut self) -> &glow::Context {
        self.imgui_renderer.gl_context()
    }

    pub fn set_clear_color(&mut self, color: Color) {
        unsafe {
            self.imgui_renderer
                .gl_context()
                .clear_color(color.r, color.g, color.b, color.a)
        };
    }

    pub fn update_delta_time(&mut self, duration: std::time::Duration) {
        self.imgui_context.io_mut().update_delta_time(duration);
    }

    pub fn request_redraw(&mut self) {
        self.winit_platform
            .prepare_frame(self.imgui_context.io_mut(), self.window_context.window())
            .unwrap();
        self.window_context.window().request_redraw();
    }

    pub fn render<F: FnOnce(&mut imgui::Ui)>(&mut self, build_ui: F) {
        unsafe { self.gl().clear(glow::COLOR_BUFFER_BIT) };

        let ui = self.imgui_context.frame();
        build_ui(ui);

        self.winit_platform
            .prepare_render(ui, self.window_context.window());
        let draw_data = self.imgui_context.render();

        self.imgui_renderer.render(draw_data).unwrap();
        self.window_context.swap_buffers().unwrap();
    }

    pub fn handle_event(&mut self, event: glutin::event::Event<()>) {
        self.winit_platform.handle_event(
            self.imgui_context.io_mut(),
            self.window_context.window(),
            &event,
        );
    }
}
