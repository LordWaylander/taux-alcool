use iced::widget::{row, radio, container};
use iced::Element;

#[derive(Default)]
pub struct SexeComponent {
    pub value: Option<Choice>
}

#[derive(Debug, Clone, Copy)]
pub enum SexeMessage {
    RadioSelected(Choice)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Femminin,
    Masculin
}

impl SexeComponent {
    pub fn view(&self) -> Element<SexeMessage> {

        let fem = container(
        radio(
            "Femminin",
            Choice::Femminin,
            self.value,
            SexeMessage::RadioSelected
        ).spacing(5)).padding(5);

        let masc = container(
            radio(
            "Masculin",
            Choice::Masculin,
            self.value,
            SexeMessage::RadioSelected
        ).spacing(5)).padding(5);

        row![
            fem, masc
        ]
        .padding(5)
        .width(500)
        .into()
    }
}