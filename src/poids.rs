use iced::widget::{text_input, row};
use iced::Element;

#[derive(Default)]
pub struct PoidsComponent {
    pub value: String
}

#[derive(Debug, Clone)]
pub enum PoidsMessage {
    ContentChanged(String)
}

impl PoidsComponent {
    pub fn view(&self) -> Element<PoidsMessage>{

        row![
            text_input("Entrez votre poids (Kg)", &self.value)
            .on_input(PoidsMessage::ContentChanged)
        ]
        .padding(5)
        .into()
    }
}