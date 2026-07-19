use glfw;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,

    Button4,
    Button5,
    Button6,
    Button7,
    Button8,

    COUNT,
}

pub const MOUSE_BUTTON_COUNT: usize = MouseButton::COUNT as usize;

/// Convert a GLFW mouse button into the engine's MouseButton.
pub fn convert_mouse(button: glfw::MouseButton) -> Option<MouseButton> {
    use glfw::MouseButton::*;

    match button {
        Button1 => Some(MouseButton::Left),
        Button2 => Some(MouseButton::Right),
        Button3 => Some(MouseButton::Middle),

        Button4 => Some(MouseButton::Button4),
        Button5 => Some(MouseButton::Button5),
        Button6 => Some(MouseButton::Button6),
        Button7 => Some(MouseButton::Button7),
        Button8 => Some(MouseButton::Button8),

        _ => None,
    }
}