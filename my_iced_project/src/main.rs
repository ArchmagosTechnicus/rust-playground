use iced::{button, Button, Column, Sandbox, Settings, Text};

#[derive(Default)]
pub struct Counter {
    buttons: Vec<(button::State, String, fn(i64) -> i64)>,
    value: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ButtonPressed(usize),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        let buttons: Vec<(button::State, String, fn(i64) -> i64)> = vec![
            (button::State::new(), String::from("Increment"), |value| value + 1),
            (button::State::new(), String::from("Decrement"), |value| value - 1),
            (button::State::new(), String::from("Square"), |value| value * value),
            // Add new buttons here
        ];

        Self {
            buttons,
            value: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ButtonPressed(index) => {
                if let Some((_, _, operation)) = self.buttons.get(index) {
                    self.value = operation(self.value);
                }
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let buttons: Vec<_> = self
            .buttons
            .iter_mut()
            .enumerate()
            .map(|(index, (button_state, label, _))| {
                Button::new(button_state, Text::new(label.clone()))
                    .on_press(Message::ButtonPressed(index))
            })
            .collect();

        Column::new()
            .push(
                buttons
                    .into_iter()
                    .fold(Column::new(), |column, button| column.push(button)),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .into()
    }
}

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}