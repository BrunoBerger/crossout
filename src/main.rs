use iced::executor;
use iced::widget;
use iced::widget::{checkbox, column, container, button, text};
use iced::{Application, Command, Element, Length, Settings, Theme};

const OFFSET: usize = 9;

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Default)]
struct MyApp {
    default_checkbox: bool,
    counter: u32,
    numbers: Vec<u8>,
    mask: Vec<u8>
}
impl MyApp {
    fn new() -> Self {
        MyApp { 
            default_checkbox: false, 
            numbers: vec![
                1,2,3,4,5,6,7,8,9,
                1,1,1,2,1,3,1,4,1,
                5,1,6,1,7,1,8,1,9, 1,1,1,1], 
            // numbers: (1..=9).chain(11..=19).collect::<Vec<_>>(), // eh
            mask: vec![0; 27],
            counter: 0}
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DefaultChecked(bool),
    NewGame,
    NumberPressed,
}

impl Application for MyApp {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            MyApp::new(),
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Crossout Game")
    }
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DefaultChecked(value) => self.default_checkbox = value,
            Message::NewGame => {println!("New Game")},
            Message::NumberPressed => {},
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        println!("View updated");
        let default_checkbox = checkbox("Default", self.default_checkbox, Message::DefaultChecked);
        let new_game_button = button("New Game").on_press(Message::NewGame);

        let mut number_col = column![];
        let mut new_row = widget::Row::new();
        
        for (i, n) in self.numbers.iter().enumerate() {
            let button_content = widget::text(n);
            let new_button = button(button_content).on_press(Message::NumberPressed);
            new_row = new_row.push(new_button);
            if i % OFFSET == OFFSET-1 {
                number_col = number_col.push(new_row);
                new_row = widget::Row::new();
            }
        }
        // add unfinished row. TODO: see if adding empty row is bad
        if self.numbers.len() % OFFSET != 0 {
            number_col = number_col.push(new_row);
        }


        let collumn = column![
            default_checkbox,
            new_game_button,
            // test_row,
            number_col,
            text(self.counter).size(20),
            ].spacing(10);

        container(collumn)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}