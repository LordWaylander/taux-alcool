use iced::widget::{pick_list, text_input, row, button, container};
use iced::Element;

#[derive(Default, Clone)]
pub struct ConsommationComponent {
    pub alcool: Option<Alcool>,
    pub volume_percent: u32,
    pub volume_ml: u32,
    pub quantity: String
}

#[derive(Debug, Clone)]
pub enum ConsommationMessage {
    AlcoolSelected(Alcool),
    ContentChanged(String),
    Delete
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alcool {
    Doux,
    Vin,
    Aperitif,
    Fort 
}

impl ConsommationComponent {
    pub fn view(&self) -> Element<ConsommationMessage> {
        let alcool = [
            Alcool::Doux,
            Alcool::Vin,
            Alcool::Aperitif,
            Alcool::Fort
        ];

        row![
            container(
                pick_list(
                    alcool,
                    self.alcool,
                    ConsommationMessage::AlcoolSelected
                )
                .width(250)
                .placeholder("Selectionnez le type d'alcool bu")
            ).padding([0,5]),

            container(
                text_input("Nombre de verres bu", &self.quantity)
                .width(200)
                .on_input(ConsommationMessage::ContentChanged)
            ).padding([0,5]),

            button("-").on_press(ConsommationMessage::Delete)
            
        ]
        .padding(5)
        .into()
    }
}

impl std::fmt::Display for Alcool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Doux => "Alcool doux (< 8%)",
            Self::Vin => "Vin (8% > 15%)",
            Self::Aperitif => "Apéritif (15% > 25%)",
            Self::Fort => "Alcool fort (30% > 60%)",
        })
    }
}