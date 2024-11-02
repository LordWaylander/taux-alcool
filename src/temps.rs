use iced::widget::{text_input, row};
use iced::Element;

#[derive(Default)]
pub struct TempsComponent {
    pub value: String
}

#[derive(Debug, Clone)]
pub enum TempsMessage {
    ContentChanged(String)
}

impl TempsComponent {
    pub fn view(&self) -> Element<TempsMessage> {
        row![
            text_input("Temps écoulé depuis la dernière consommation ", &self.value)
            .on_input(TempsMessage::ContentChanged)
        ]
        .padding(5)
        .into()
    }
}