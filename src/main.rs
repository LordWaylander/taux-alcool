use iced::widget::{self, button, column, keyed_column, row, container, stack, opaque, mouse_area, center};
use iced::{Color, Element, Task, Theme};
use iced::alignment::Horizontal::Center;

const COEFF_DIFF_FEMME: f32 = 0.6;
const COEFF_DIFF_HOMME: f32 = 0.7;
const VIT_ELIMIN_ALCOOL: f32 = 0.15;

mod sexe;
mod poids;
mod temps;
mod consommation;
mod modal_consommation;
mod modal_error;

use sexe::{SexeComponent, SexeMessage};
use poids::{PoidsComponent, PoidsMessage};
use temps::{TempsComponent, TempsMessage};
use consommation::{ConsommationComponent, ConsommationMessage};
use modal_consommation::{ModalConsommationComponent, ModalConsommationMessage};
use modal_error::{ModalErrorComponent, ModalErrorMessage};

struct App {
    sexe_component: SexeComponent,
    poids_component: PoidsComponent,
    temps_component: TempsComponent,
    consommation_component: Vec<ConsommationComponent>,
    modal_consommation_component: ModalConsommationComponent,
    modal_error_component: ModalErrorComponent,
    show_modal: bool,
    show_modal_error: bool
}

#[derive(Debug, Clone)]
enum Message {
    SexeMessage(SexeMessage),
    PoidsMessage(PoidsMessage),
    TempsMessage(TempsMessage),
    ConsommationMessage(usize, ConsommationMessage),
    ModalConsommationMessage(ModalConsommationMessage),
    ModalErrorMessage(ModalErrorMessage),
    Calcul,
    Reset,
    AddConsomation,
}

impl Default for App {
    fn default() -> Self {
        Self {
            sexe_component: SexeComponent::default(),
            poids_component: PoidsComponent::default(),
            temps_component: TempsComponent::default(),
            consommation_component: vec![ConsommationComponent::default()],
            modal_consommation_component: ModalConsommationComponent::default(),
            modal_error_component: ModalErrorComponent::default(),
            show_modal: false,
            show_modal_error: false
        }
    }
}

impl App {
    fn view(&self) -> Element<Message> {

        let quantity_component: Element<Message> = keyed_column(
            self.consommation_component.iter().enumerate().map(|(i, quanti)| {
                (
                    i,
                    quanti.view().map(move |q| {
                        Message::ConsommationMessage(i, q)
                    })
                )
            })
        ).into();

        let content = container(
            column![
                self.sexe_component.view().map(Message::SexeMessage),
                self.poids_component.view().map(Message::PoidsMessage),
                self.temps_component.view().map(Message::TempsMessage),
                
                row![
                    quantity_component,
                ],
                container(
                    button("Ajouter une consommation").padding(5).on_press(Message::AddConsomation),
                ).padding(5),
                container(
                    row![
                        container(
                            button("Calculer").padding(5).on_press(Message::Calcul)
                        ).padding(5),
                        container(
                            button("Annuler").padding(5).on_press(Message::Reset)
                        ).padding(5)
                    ]
                )
            ].align_x(Center)
        )
        .width(500)
        .height(800);

        if self.show_modal {
            let modal_consommation = self.modal_consommation_component.view().map(Message::ModalConsommationMessage);

            modal(content, modal_consommation, Message::ModalConsommationMessage(ModalConsommationMessage::HideModal))
        } else if self.show_modal_error{
            let modal_error = self.modal_error_component.view().map(Message::ModalErrorMessage);

            modal(content, modal_error, Message::ModalConsommationMessage(ModalConsommationMessage::HideModal))

        }else {
            content.into()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::PoidsMessage(PoidsMessage::ContentChanged(content)) => {
                self.poids_component.value = content;
                Task::none()
            }
            Message::SexeMessage(SexeMessage::RadioSelected(option)) => {
                self.sexe_component.value = Some(option);
                Task::none()
            }
            Message::TempsMessage(TempsMessage::ContentChanged(content)) => {
                self.temps_component.value = content;
                Task::none()
            }
            Message::ConsommationMessage(i,ConsommationMessage::AlcoolSelected(content)) => {
                self.consommation_component[i].alcool = Some(content);
                match content {
                    consommation::Alcool::Doux => {
                        self.consommation_component[i].volume_percent = 8;
                        self.consommation_component[i].volume_ml = 250;
                    },
                    consommation::Alcool::Vin => {
                        self.consommation_component[i].volume_percent = 15;
                        self.consommation_component[i].volume_ml = 125;
                    },
                    consommation::Alcool::Aperitif => {
                        self.consommation_component[i].volume_percent = 25;
                        self.consommation_component[i].volume_ml = 80;
                    },
                    consommation::Alcool::Fort => {
                        self.consommation_component[i].volume_percent = 40;
                        self.consommation_component[i].volume_ml = 40;
                    },
                }
                Task::none()
            },
            Message::ConsommationMessage(i, ConsommationMessage::Delete ) => {
                if self.consommation_component.len() > 1 {
                    self.consommation_component.remove(i);
                }
                Task::none()
                
            }
            Message::ConsommationMessage(i, ConsommationMessage::ContentChanged(quantity)) => {
                self.consommation_component[i].quantity = quantity;
                Task::none()
            }
            Message::AddConsomation => {
                self.consommation_component.push(ConsommationComponent::default());
                Task::none()
            }
            Message::ModalConsommationMessage(ModalConsommationMessage::HideModal) => {
                self.show_modal = false;
                Task::none()
            }
            Message::ModalErrorMessage(ModalErrorMessage::HideModal) => {
                self.show_modal_error = false;
                Task::none()
            }
            Message::Calcul => {
                let mut poids = 0;
                let mut temps = 0;
                let mut taux_alcool: f32 = 0.0;

                for consommation in self.consommation_component.clone().into_iter() {
                    let mut volume = 0;

                    if consommation.alcool == None {
                        self.modal_error_component.error = "Une consomation doit être choisie".to_string();
                        self.show_modal_error = true;
                    }

                    match consommation.quantity.parse::<u32>() {
                        Ok(q) => {
                            volume = q * consommation.volume_ml
                        }
                        Err(e) => {
                            self.modal_error_component.error = e.to_string();
                            self.show_modal_error = true;
                            //panic!("{e}")
                        }
                    }

                    taux_alcool += volume as f32 * (consommation.volume_percent as f32 / 100 as f32) * 0.8;
                }

                match self.poids_component.value.parse::<u32>() {
                    Ok(pds) => {
                        poids = pds;
                    }
                    Err(e) => {
                        self.modal_error_component.error = e.to_string();
                        self.show_modal_error = true;
                        //panic!("{e}")
                    }
                };

                match self.temps_component.value.parse::<u32>() {
                    Ok(temp) => {
                        temps = temp;
                    }
                    Err(e) => {
                        self.modal_error_component.error = e.to_string();
                        self.show_modal_error = true;
                        //panic!("{e}")
                    }
                };

                match self.sexe_component.value {
                    Some(value) => {
                        match value {
                            sexe::Choice::Masculin => {
                                let mut t = (((taux_alcool / (poids as f32 * COEFF_DIFF_HOMME)) - temps as f32 * VIT_ELIMIN_ALCOOL)*100.0).round()/100.0;

                                if t <= 0.0 {
                                    t = 0.0;
                                }
                                self.modal_consommation_component.value = t
                            }
                            sexe::Choice::Femminin => {
                                let mut t = (((taux_alcool / (poids as f32 * COEFF_DIFF_FEMME)) - temps as f32 * VIT_ELIMIN_ALCOOL)*100.0).round()/100.0;

                                if t <= 0.0 {
                                    t = 0.0;
                                }
                                self.modal_consommation_component.value = t
                            }
                        }
                    }
                    None => {
                        self.modal_error_component.error = "Un sexe doit être choisi".to_string();
                        self.show_modal_error = true;
                        //panic!("Un sexe doit être choisi")
                    }
                }

                if self.show_modal_error == false {
                    self.show_modal = true;
                }

                widget::focus_next()
            }
            Message::Reset => {
                self.poids_component.value = String::new();
                self.sexe_component.value = None;
                self.temps_component.value = String::new();
                self.consommation_component = vec![ConsommationComponent::default()];
                Task::none()
            }
        }        
    }
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    action: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(action)
        )
    ]
    .into()
}

fn main() -> iced::Result {
    iced::application("calcul taux alcool", App::update, App::view)
    .theme(|_| Theme::Dark)
    .window_size((500.0, 800.0))
    .run()
}
