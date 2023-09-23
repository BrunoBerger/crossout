use iced::executor;
use iced::widget;
use iced::widget::row;
use iced::widget::{checkbox, column, container, button, text};
use iced::{Application, Command, Element, Length, Settings, Theme};

const OFFSET: usize = 3;

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
        MyApp { default_checkbox: false, numbers: vec![1,2,3,4,5,6], mask: vec![0,0,0,0] ,counter: 0}
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
            Message::NewGame => {println!("test")},
            Message::NumberPressed => {},
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        println!("View updated");
        let default_checkbox = checkbox("Default", self.default_checkbox, Message::DefaultChecked);
        let new_game_button = button("Press me!").on_press(Message::NewGame);

        let mut number_col = column![];

        let mut irow = widget::Row::new();
    
        
        for (i, n) in self.numbers.iter().enumerate() {
            let new_button = button("n").on_press(Message::NumberPressed);
            irow = irow.push(new_button);
            if i % OFFSET <= 0 {
                number_col = number_col.push(irow);
                irow = widget::Row::new();
            }
        }
        number_col = number_col.push(irow);
        
        // let mut test_row = row![
        //     button("Pstese").on_press(Message::NewGame),
        //     button("Presssefef").on_press(Message::NewGame),
        // ];

        // let sub_col = widget::column(number_content);

        let collumn = column![
            default_checkbox,
            new_game_button,
            // test_row,
            number_col,
            text(self.counter).size(50),
            ].spacing(10);


        container(collumn)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}