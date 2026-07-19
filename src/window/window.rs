use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};
use crate::math::IVec2;


pub struct Window {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>
}

impl Window {

    pub fn new(constants: &WindowConstants) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut window, events) = 
            glfw.create_window(constants.width, constants.height, constants.title, glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window");

        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_close_polling(true);

        Self{ glfw, window, events }
    }

    pub fn get_size(&self) -> IVec2 {
        let (width, height): (i32, i32) = self.window.get_size();
        IVec2::new(width, height)
    }
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.window.set_size(width, height);
    }
    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }


    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
    
    pub fn poll_events(&mut self, input: &mut Input) {
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            input.process_event(event);
        }
    }
}