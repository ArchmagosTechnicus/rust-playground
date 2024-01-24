use iced::{button, Button, Column, Sandbox, Settings, Text};

#[derive(Default)]
pub struct Counter {
    increment_button: button::State,
    decrement_button: button::State,
    square_button: button::State,
    value: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
    Square,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::Square => {
                self.value *= self.value;
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::Increment),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::Decrement),
            )
            .push(
                Button::new(&mut self.square_button, Text::new("Square"))
                    .on_press(Message::Square),
            )
            .into()
    }
}

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}