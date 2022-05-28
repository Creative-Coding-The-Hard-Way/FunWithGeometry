use ::anyhow::Result;

use crate::graphics::{
    immediate_mode_graphics::triangles::Frame,
    ui::{
        primitives::Dimensions,
        widgets::{Element, Widget},
        Input, InternalState,
    },
    Vec2,
};

/// Messages of this type are used by Composite widgets to differentiate
/// between "external" events which are meant to pass-through to user code
/// and "internal" events which are used to manage the widget's state.
#[derive(Debug, Copy, Clone)]
pub enum ComposedMessage<I, E> {
    Internal(I),
    External(E),
}

/// An Element decorator which automatically wraps all underlying events into
/// ['ComposedMessage'] external values.
pub struct ComposedElement<E>(pub Element<E>);

impl<I, E> Widget<ComposedMessage<I, E>> for ComposedElement<E> {
    /// Allow the underlying Element to handle window events. Any resulting
    /// messages are automatically wrapped into ComposedElement::External.
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<ComposedMessage<I, E>>> {
        self.0
            .handle_event(internal_state, input, event)
            .map(|opt| opt.map(ComposedMessage::External))
    }

    /// Allow the underlying Element to draw itself to the frame.
    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()> {
        self.0.draw_frame(internal_state, frame)
    }

    /// Allow the underlying Element to compute its own dimensions.
    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        self.0.dimensions(internal_state, max_size)
    }

    /// Allow the underlying Element set its own top left position.
    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        self.0.set_top_left_position(internal_state, position);
    }
}

impl<I, E> From<Element<E>> for Element<ComposedMessage<I, E>>
where
    E: 'static,
{
    fn from(element: Element<E>) -> Self {
        Element::new(ComposedElement(element))
    }
}
