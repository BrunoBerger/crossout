use iced::executor;
use iced::theme;
use iced::widget;
use iced::widget::{checkbox, column, container, button, text};
use iced::{Application, Color, Command, Element, Length, Theme};

const OFFSET: usize = 9;
// const MASKED_STYLE = style
const START_SET: [u8; 27] = [
    1,2,3,4,5,6,7,8,9,
    1,1,1,2,1,3,1,4,1,
    5,1,6,1,7,1,8,1,9];

pub fn main() -> iced::Result {
    let settings = iced::Settings {
        window: iced::window::Settings {
            size: (300, 600),
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    };
    MyApp::run(settings)
    // MyApp::run(Settings::default())
}

#[derive(Default)]
struct MyApp {
    default_checkbox: bool,
    numbers: Vec<u8>,
    already_used: Vec<bool>,
    selection_1: i64, // negative if no selection
}
impl MyApp {
    fn new() -> Self {
        // numbers: (1..=9).chain(11..=19).collect::<Vec<_>>(), // eh
        // let start_set = vec![1,2,3,4,5,6,7,8,9];
        MyApp { 
            default_checkbox: false, 
            already_used: vec![false; START_SET.len()],
            numbers: START_SET.to_vec(), 
            selection_1: -1,
        }
    }
    fn valid_move(&self, selection_2: usize) -> bool {

        let val1 = self.numbers[self.selection_1 as usize];
        let val2 = self.numbers[selection_2];
        if val1 + val2 != 10 && val1 != val2 {
            return false;
        }


        let idx1 = std::cmp::min(self.selection_1 as usize, selection_2) as i64;
        let idx2 = std::cmp::max(self.selection_1 as usize, selection_2) as i64;

        let mut adjacent_valid = true;
        let mut vertical_valid = (idx2-idx1) % OFFSET as i64 == 0;

        // println!("1:{idx1} 2:{idx2}");
        for next_idx in idx1+1..idx2 {
            // println!("nextidx: {next_idx}");
            if self.already_used[next_idx as usize] == false {
                adjacent_valid = false;
            }
            if (next_idx-idx1) % OFFSET as i64 == 0 as i64 {
                if self.already_used[next_idx as usize] == false {
                    vertical_valid = false;
                }
            }
        }
        if adjacent_valid || vertical_valid {
            return true
        }
        
        //Check start-end edge case
        let start = &self.already_used[0..idx1 as usize];
        let end = &self.already_used[(idx2+1) as usize .. ];
        [start, end].concat().iter().all(|&i| i == true)
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DefaultChecked(bool),
    NewGame,
    FinishedTurn,
    NumberPressed(usize),
}

// struct NumberButtonSyle {
//     color: Color
// }
// impl button::StyleSheet for NumberButtonSyle {
//     fn active(&self, style: &Self::Style) -> button::Appearance {
//         button::Appearance {
//             background: Some(iced::Background::Color(self.color)),
//             ..Default::default()
//         }
//     }
// }

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
            Message::NewGame => {
                *self = MyApp::new();
            },
            Message::FinishedTurn => {
                // yeah idk
                let mut remaning_numbers: Vec<_> = self.already_used.iter().zip(self.numbers.iter())
                    .filter_map(|(used, number)| if ! *used {Some(*number)} else {None})
                    .collect();
                let new_len = self.already_used.len() + remaning_numbers.len();
                self.already_used.resize(new_len, false);
                self.numbers.append(&mut remaning_numbers);
            },
            Message::NumberPressed(button_idx) => {
                // If no selection is yet made
                if self.selection_1 < 0 {
                    self.selection_1 = button_idx as i64;
                } 
                else {
                    // deselect if clicked again
                    if button_idx as i64 == self.selection_1 {
                        self.already_used[self.selection_1 as usize] = false;
                        self.selection_1 = -1;
                    } // if 2 buttons are selected
                    else if self.valid_move(button_idx) {
                        self.already_used[button_idx] = true;
                        self.already_used[self.selection_1 as usize] = true;
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

        let h_spacing = 1;
        let v_spacing = h_spacing;
        let mut number_col = column![].spacing(v_spacing);
        let mut new_row = widget::Row::new().spacing(h_spacing);
        
        for (i, n) in self.numbers.iter().enumerate() {
            let button_content = widget::text(n);
            let mut new_button = button(button_content)
                // .width(25).height(25)
                ;

            if self.already_used[i] == false {
                new_button = new_button.on_press(Message::NumberPressed(i));
            }
            if i as i64 == self.selection_1 {
                new_button = new_button.style(theme::Button::Positive);
            }

            new_row = new_row.push(new_button);
            if i % OFFSET == OFFSET-1 {
                number_col = number_col.push(new_row);
                new_row = widget::Row::new().spacing(h_spacing);
            }
        }
        // add unfinished row. TODO: see if adding empty row is bad
        if self.numbers.len() % OFFSET != 0 {
            number_col = number_col.push(new_row);
        }


        let collumn = column![
            widget::row![default_checkbox, text(self.selection_1).size(16)].spacing(10),
            widget::row![new_game_button, widget::Space::with_width(10), finished],
            widget::scrollable(number_col),
            ].spacing(10);

        container(collumn)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            // .center_y()
            .padding(10)
            .into()
    }
}
