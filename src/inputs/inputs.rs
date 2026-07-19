use glfw::{Action, WindowEvent};
use crate::math::Vec2;
use crate::inputs::key::Key;
use crate::inputs::mouse_button::MouseButton;

const KEY_COUNT: usize = Key::COUNT as usize;
const MOUSE_COUNT: usize = MouseButton::COUNT as usize;

pub struct Inputs
{
    keys_held: [bool; KEY_COUNT],
    keys_pressed: [bool; KEY_COUNT],
    keys_released: [bool; KEY_COUNT],

    mouse_held: [bool; MOUSE_COUNT],
    mouse_pressed: [bool; MOUSE_COUNT],
    mouse_released: [bool; MOUSE_COUNT],

    mouse_position: Vec2
}

impl Inputs {
    pub fn new() -> Self {
        Self{
            keys_held: [false; KEY_COUNT],
            keys_pressed: [false; KEY_COUNT],
            keys_released: [false; KEY_COUNT],
            mouse_held: [false; MOUSE_COUNT],
            mouse_pressed: [false; MOUSE_COUNT],
            mouse_released: [false; MOUSE_COUNT],
            mouse_position: Vec2::ZERO
        }
    }

    pub fn key_held(&self, key: Key) -> bool {
        self.keys_held[key as usize]
    }
    pub fn key_pressed(&self, key: Key) -> bool {
        self.keys_pressed[key as usize]
    }
    pub fn key_released(&self, key: Key) -> bool {
        self.keys_released[key as usize]
    }

    pub fn mouse_held(&self, mouse_button: MouseButton) -> bool {
        self.mouse_held[mouse_button as usize]
    }
    pub fn mouse_pressed(&self, mouse_button: MouseButton) -> bool {
        self.mouse_pressed[mouse_button as usize]
    }
    pub fn mouse_released(&self, mouse_button: MouseButton) -> bool {
        slef.mouse_released[mouse_button as usize]
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn begin_frame(&mut self) {
        self.keys_pressed.fill(false);
        self.keys_released.fill(false);

        self.mouse_pressed.fill(false);
        self.mouse_released.fill(false);
    }

    pub fn process_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::Key(key, _, Action::Press, _) => {
                if let Some(key) = convert_key(key) {
                    let i: usize = key as usize;
                    self.keys_held[i] = true;
                    self.keys_pressed[i] = true;
                }
            }

            WindowEvent::Key(key, _, Action::Release, _) => {
                if let Some(key) = convert_key(key) {
                    let i: usize = key as usize;
                    self.keys_held[i] = false;
                    self.keys_released[i] = true;
                }
            }

            WindowEvent::MouseButton(button, _, Action::Press, _) => {
                if let Some(button) = convert_mouse(button) {
                    let i: usize = button as usize;
                    self.mouse_held[i] = true;
                    self.mouse_pressed[i] = true;
                }
            }

            WindowEvent::MouseButton(button, _, Action::Release, _) => {
                if let Some(button) = convert_mouse(button) {
                    let i: usize = button as usize;
                    self.mouse_held[i] = false;
                    self.keys_released[i] = true;
                }
            }

            WindowEvent::CursorPos(x, y) => {
                self.mouse_position = Vec2::new(x as f32, y as f32);
            }

            _ => {}
        }
    }
}