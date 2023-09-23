use iced::executor;
use iced::widget::{checkbox, column, container};
use iced::{Application, Command, Element, Length, Settings, Theme};


pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Default)]
struct MyApp {
    default_checkbox: bool,
    numbers: Vec<u8>,
    mask: Vec<u8>
}
impl MyApp {
    fn new() -> Self {
        MyApp { default_checkbox: false, numbers: vec![1,2,3,4], mask: vec![0,0,0,0] }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DefaultChecked(bool),
    NewGame(),
}

impl Application for MyApp {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            // Self::default(),
            MyApp::new(),
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Checkbox - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DefaultChecked(value) => self.default_checkbox = value,
            Message::NewGame() => {},
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let default_checkbox =
            checkbox("Default", self.default_checkbox, Message::DefaultChecked);

        for n in &self.numbers {
            println!("{n}");
        }
        
        let content = column![default_checkbox].spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}