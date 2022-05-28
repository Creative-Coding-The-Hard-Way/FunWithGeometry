use ::anyhow::Result;

use crate::graphics::{
    immediate_mode_graphics::triangles::Frame,
    ui::{primitives::Dimensions, widgets::Widget, Input, InternalState},
    Vec2,
};

/// A marker trait used by non-element widgets to automatically convert to an
/// Element when needed.
pub trait AutoElement {}

/// An Element is a type-erased widget.
/// Elements allow UI objects to hold a variety of Widget implementations and
/// dynamically dispatch function calls as needed.
///
pub struct Element<Message> {
    pub(crate) widget: Box<dyn Widget<Message>>,
}

impl<Message> Element<Message> {
    pub fn new(widget: impl Widget<Message> + 'static) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }
}

impl<Message> Widget<Message> for Element<Message> {
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<Message>> {
        self.widget.handle_event(internal_state, input, event)
    }

    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()> {
        self.widget.draw_frame(internal_state, frame)
    }

    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        self.widget.dimensions(internal_state, max_size)
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        self.widget.set_top_left_position(internal_state, position)
    }
}

impl<Message, W: AutoElement + Widget<Message>> From<W> for Element<Message>
where
    Message: 'static,
    W: 'static,
{
    fn from(widget: W) -> Self {
        Element::new(widget)
    }
}
