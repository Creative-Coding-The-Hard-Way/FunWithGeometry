mod demo_ui;

use ::{
    anyhow::Result,
    fwg::{
        demo::{run_application, State},
        graphics::{
            asset_loader::AssetLoader,
            glfw_window::GlfwWindow,
            immediate_mode_graphics::triangles::Frame,
            math::projections,
            timing::FrameRateLimit,
            ui::UI,
            vulkan::{MemoryAllocator, RenderDevice},
            Mat4,
        },
    },
    std::sync::Arc,
};

use self::demo_ui::{ExampleUi, UIMessage};

struct Example {
    ui: UI<ExampleUi>,
    app_camera: Mat4,
}

impl Example {
    fn projection(aspect_ratio: f32) -> Mat4 {
        let height = 10.0;
        let width = height * aspect_ratio;
        projections::ortho(
            -0.5 * width,
            0.5 * width,
            -0.5 * height,
            0.5 * height,
            0.0,
            1.0,
        )
    }
}

impl State for Example {
    fn init(
        window: &mut GlfwWindow,
        fps_limit: &mut FrameRateLimit,
        asset_loader: &mut AssetLoader,
        _vk_dev: &Arc<RenderDevice>,
        _vk_alloc: &Arc<dyn MemoryAllocator>,
    ) -> Result<Self> {
        let scale = window.window.get_content_scale();

        let (w, h) = window.window.get_framebuffer_size();
        let aspect_ratio = w as f32 / h as f32;

        fps_limit.set_target_fps(60);

        Ok(Self {
            ui: UI::new(
                window.window.get_framebuffer_size().into(),
                ExampleUi::new(scale.0, asset_loader)?,
            ),
            app_camera: Self::projection(aspect_ratio),
        })
    }

    fn handle_event(
        &mut self,
        event: glfw::WindowEvent,
        window: &mut GlfwWindow,
    ) -> Result<()> {
        if Some(UIMessage::ToggleFullscreen) == self.ui.handle_event(&event)? {
            window.toggle_fullscreen()?;
        }
        if let glfw::WindowEvent::FramebufferSize(w, h) = event {
            self.app_camera = Self::projection(w as f32 / h as f32);
        }
        Ok(())
    }

    fn draw_frame(
        &mut self,
        app_frame: &mut Frame,
        ui_frame: &mut Frame,
    ) -> Result<()> {
        self.ui.draw_frame(ui_frame)?;

        app_frame.set_view_projection(self.app_camera)?;

        Ok(())
    }

    fn rebuild_swapchain_resources(
        &mut self,
        _window: &GlfwWindow,
        _framebuffer_size: (u32, u32),
    ) -> Result<()> {
        Ok(())
    }
}

impl Example {}

fn main() -> Result<()> {
    run_application::<Example>()
}
