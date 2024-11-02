use iced::widget::{container, column, text, button};
use iced::{Element, color};
use iced::alignment::Horizontal::Center;

#[derive(Default)]
pub struct ModalConsommationComponent {
    pub value : f32
}

#[derive(Debug, Clone)]
pub enum ModalConsommationMessage {
    HideModal
}

impl ModalConsommationComponent {
    
    pub fn view(&self) -> Element<ModalConsommationMessage> {
        container(
            column![
                text("Votre taux d'alcool est d'environ : ").size(24),
                text(self.value).size(24),
                container(
                    text("ATTENTION : ceci n'est qu'une simple estimation, votre taux peut être supérieur à celui affiché").size(24).color(color!(255,0,0))
                ).padding(5),
                button(text("OK")).on_press(ModalConsommationMessage::HideModal),
            ].align_x(Center)
        ).into()
    }
}