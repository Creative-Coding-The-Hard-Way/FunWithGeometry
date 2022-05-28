mod glfw_window_impl;
mod window_error;

use std::sync::mpsc::Receiver;

pub use self::{glfw_window_impl::GlfwWindow, window_error::WindowError};

/// GLFW uses a Receiver for accepting window events. This type alias is more
/// convenient to write/read than the full name.
pub type EventReceiver = Receiver<(f64, glfw::WindowEvent)>;
