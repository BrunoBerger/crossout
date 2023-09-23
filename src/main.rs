use iced::executor;
use iced::widget;
use iced::widget::{checkbox, column, container, button, text};
use iced::{Application, Command, Element, Length, Settings, Theme};

const OFFSET: usize = 9;
// const MASKED_STYLE = style


pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Default)]
struct MyApp {
    default_checkbox: bool,
    unused_counter: u32,
    numbers: Vec<u8>,
    mask: Vec<bool>,
    selection_1: i64, // negative if no selection
}
impl MyApp {
    fn new() -> Self {
        MyApp { 
            default_checkbox: false, 
            numbers: vec![
                1,2,3,4,5,6,7,8,9,
                1,1,1,2,1,3,1,4,1,
                5,1,6,1,7,1,8,1,9], 
            // numbers: (1..=9).chain(11..=19).collect::<Vec<_>>(), // eh
            mask: vec![false; 27],
            unused_counter: 0,
            selection_1: -1,
        }
    }
    fn valid_move(&self, selection_2: usize) -> bool {
        let idx1 = std::cmp::min(self.selection_1 as usize, selection_2) as i64;
        let idx2 = std::cmp::max(self.selection_1 as usize, selection_2) as i64;

        let mut adjacent_blocked = false;
        let mut vertical_blocked = false;

        for next_idx in idx1..idx2 {
            // println!("{next_number}")
            if self.mask[next_idx as usize] == false {
                adjacent_blocked = true;
            }
            if (next_idx-idx1) % OFFSET as i64 == OFFSET as i64 - 1 {
            // if (next_idx-idx1) % 9 == 0 {
                if self.mask[next_idx as usize] == false {
                    vertical_blocked = true;
                }
            }
        }
        println!("adjacent blocked: {adjacent_blocked}");
        println!("vertical blocked: {vertical_blocked}");
        //Check start-end edge case


        true
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DefaultChecked(bool),
    NewGame,
    FinishedTurn,
    NumberPressed(usize),
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
            Message::NewGame => println!("New Game"),
            Message::FinishedTurn => println!("Turn over"),
            Message::NumberPressed(button_idx) => {
                println!("Pressed button {button_idx}");
                // If no selection is yet made
                if self.selection_1 < 0 {
                    self.selection_1 = button_idx as i64;
                    self.mask[self.selection_1 as usize] = true
                } 
                else {
                    // deselect if clicked again
                    if button_idx as i64 == self.selection_1 {
                        // TODO this cant be reached as button is deactivated and cant be pressed again
                        self.mask[self.selection_1 as usize] = false;
                        self.selection_1 = -1;
                        println!("UNSELECTED")
                    } // if 2 buttons are selected
                    else if self.valid_move(button_idx) {
                        self.mask[button_idx] = true;
                        self.mask[self.selection_1 as usize] = true;
                        self.selection_1 = -1;
                    }
                }

            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let default_checkbox = checkbox("tmp", self.default_checkbox, Message::DefaultChecked);
        let new_game_button = button("New Game").on_press(Message::NewGame);
        let finished = button("End Turn").on_press(Message::FinishedTurn);

        let mut number_col = column![];
        let mut new_row = widget::Row::new();
        
        for (i, n) in self.numbers.iter().enumerate() {
            let button_content = widget::text(n);
            let mut new_button = button(button_content);
            if self.mask[i] == false {
                new_button = new_button.on_press(Message::NumberPressed(i));
            }

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
            widget::row![new_game_button, widget::Space::with_width(10), finished],
            number_col,
            text(self.selection_1).size(20),
            ].spacing(10);

        container(collumn)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}