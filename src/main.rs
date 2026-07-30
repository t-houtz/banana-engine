#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod vk_rendering;

use anyhow::Result;
use winit::event_loop::EventLoop;

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window
    let event_loop = EventLoop::new()?;
    let mut handler = vk_rendering::app_handler::AppHandler { window: None, app: None, minimized: false };

    event_loop.run_app(&mut handler)?;

    Ok(())
}
