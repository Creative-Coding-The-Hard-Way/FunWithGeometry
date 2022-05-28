use anyhow::Result;
use fwg::graphics::{
    asset_loader::AssetLoader,
    ui::{widgets::prelude::*, UIState},
    vec4,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UIMessage {
    ToggleFullscreen,
}

pub struct ExampleUi {
    em: f32,
    font: Font,
    is_fullscreen: bool,
    pub angle: f32,
}

impl ExampleUi {
    pub fn new(
        content_scale: f32,
        asset_loader: &mut AssetLoader,
    ) -> Result<Self> {
        let em = 16.0 * content_scale;
        let font = Font::from_font_file(
            "assets/Roboto-Regular.ttf",
            1.0 * em,
            asset_loader,
        )?;
        Ok(Self {
            em,
            font,
            is_fullscreen: false,
            angle: 0.0,
        })
    }
}

impl UIState for ExampleUi {
    type Message = UIMessage;

    fn view(&self) -> Element<Self::Message> {
        let message = if self.is_fullscreen {
            "Windowed"
        } else {
            "Fullscreen"
        };
        let fullscreen_button = text_button(&self.font, message)
            .on_click(UIMessage::ToggleFullscreen)
            .color(vec4(1.0, 1.0, 1.0, 0.0))
            .hover_color(vec4(1.0, 1.0, 1.0, 0.1))
            .pressed_color(vec4(1.0, 1.0, 1.0, 0.5))
            .container()
            .border(1.0, vec4(0.0, 0.0, 0.0, 0.75), 0)
            .padding(0.5 * self.em);

        align(fullscreen_button)
            .alignment(HAlignment::Right, VAlignment::Top)
            .into()
    }

    fn update(&mut self, message: &UIMessage) {
        match *message {
            UIMessage::ToggleFullscreen => {
                self.is_fullscreen = !self.is_fullscreen;
            }
        }
    }
}
