use iced::{alignment::{Horizontal, Vertical}, widget::{Column, Container}, Element, Settings, Application, Command, Theme, executor, Length};
use iced::widget::{Row, Scrollable, Text};
use iced_aw::{TabLabel, Tabs};
use login::{LoginMessage, LoginTab};

mod ferris;
mod login;
use ferris::{FerrisMessage, FerrisTab};

fn main() -> iced::Result {
    TabBarExample::run(Settings::default())
}

struct TabBarExample {
    active_tab: usize,
    login_tab: LoginTab,
    ferris_tab: FerrisTab,
}

#[derive(Clone, Debug)]
enum Message {
    TabSelected(usize),
    Login(LoginMessage),
    Ferris(FerrisMessage),
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum TabId {
    Login,
    Ferris,
}

impl Application for TabBarExample {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (TabBarExample {
            active_tab: 0,
            login_tab: LoginTab::new(),
            ferris_tab: FerrisTab::new(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("TabBar Example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Login(message) => self.login_tab.update(message),
            Message::Ferris(message) => self.ferris_tab.update(message),
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let scrollable = Scrollable::new(self.login_tab.view())
            .height(Length::Fill);

        let tabs = Tabs::new(Message::TabSelected)
            .push(
                    TabId::Login as usize,
                self.login_tab.tab_label(),
                scrollable)
            .push(
                    TabId::Ferris as usize,
                self.ferris_tab.tab_label(),
                self.ferris_tab.view())
            .set_active_tab(&self.active_tab);

        Column::new()
            .push(Row::new().push(Text::new("Header")))
            .push(tabs)
            .push(Row::new().push(Text::new("Footer")))
            .padding(10)
            .into()
    }
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
