use iced::widget::{container, column, text, button};
use iced::Element;
use iced::alignment::Horizontal::Center;

#[derive(Default)]
pub struct ModalErrorComponent {
    pub error : String
}

#[derive(Debug, Clone)]
pub enum ModalErrorMessage {
    HideModal
}

impl ModalErrorComponent {
    
    pub fn view(&self) -> Element<ModalErrorMessage> {
        container(
            column![
                text("Une erreur c'est produite : ").size(24),
                text(self.error.clone()).size(24),
                button(text("OK")).on_press(ModalErrorMessage::HideModal),
            ].align_x(Center)
        ).into()
    }
}