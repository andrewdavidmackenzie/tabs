use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, Row, Text, TextInput}, Element};
use iced::widget::Scrollable;
use iced_aw::tab_bar::TabLabel;

use crate::{Message, Tab};

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UsernameChanged(String),
    PasswordChanged(String),
    ClearPressed,
    LoginPressed,
}

pub struct LoginTab {
    username: String,
    password: String,
}

impl LoginTab {
    pub fn new() -> Self {
        LoginTab {
            username: String::new(),
            password: String::new(),
        }
    }

    pub fn update(&mut self, message: LoginMessage) {
        match message {
            LoginMessage::UsernameChanged(value) => self.username = value,
            LoginMessage::PasswordChanged(value) => self.password = value,
            LoginMessage::ClearPressed => {
                self.username = String::new();
                self.password = String::new();
            }
            LoginMessage::LoginPressed => {}
        }
    }
}

impl Tab for LoginTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Login")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let inner =
            Column::new()
                .push(
                    TextInput::new("Username", &self.username)
                        .on_input(LoginMessage::UsernameChanged)
                        .padding(10)
                        .size(32),
                )
                .push(
                    TextInput::new("Password", &self.password)
                        .on_input(LoginMessage::PasswordChanged)
                        .password(),
                )
                .push(
                    Row::new()
                        .spacing(10)
                        .push(
                            Button::new(
                                Text::new("Clear").horizontal_alignment(Horizontal::Center),
                            )
                                .on_press(LoginMessage::ClearPressed),
                        )
                        .push(
                            Button::new(
                                Text::new("Login").horizontal_alignment(Horizontal::Center),
                            )
                                .on_press(LoginMessage::LoginPressed),
                        ),
                );

        let content: Element<'_, LoginMessage> = Container::new(Scrollable::new(inner))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();

        content.map(Message::Login)
    }
}
