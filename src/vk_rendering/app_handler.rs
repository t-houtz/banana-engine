use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};
use super::App;

pub struct AppHandler {
    pub window: Option<Window>,
    pub app: Option<App>,

    pub minimized: bool
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(
            WindowAttributes::default()
                .with_title("Banana Engine")
                .with_inner_size(LogicalSize::new(1024, 768))
        ).unwrap();

        let app = unsafe {
            App::create(&window).unwrap()
        };

        self.window = Some(window);
        self.app = Some(app);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if !self.minimized {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {  
        match event {
            WindowEvent::Resized(size) => {
                if let Some(app) = &mut self.app {
                    if size.width == 0 || size.height == 0 {
                        self.minimized = true;
                    }
                    else {
                        self.minimized = false;
                        app.resized = true;
                    }
                }
            }

            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(app) = &mut self.app {
                    app.resized = true;
                }
            }

            WindowEvent::RedrawRequested => {
                if self.minimized {
                    return;
                }

                if let (Some(window), Some(app)) = (&self.window, &mut self.app) {
                    unsafe {
                        app.render(window).unwrap();
                    }
                }
            }

            WindowEvent::CloseRequested => {
                if let Some(app) = &mut self.app {
                    unsafe {
                        app.destroy();
                    }
                }

                event_loop.exit();
            }

            _ => {}
        }

    }
}