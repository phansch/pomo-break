use iced::{button, Button, Column, Element, Sandbox, Settings, Text};
use std::time::Duration;

#[derive(Default)]
struct Pomo {
    elapsed: Option<Duration>,

    // The local state of the two buttons
    cancel_button: button::State,
    start_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    StartPressed,
    CancelPressed,
}


impl Sandbox for Pomo {

    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout
        Column::new()
            .push(
                // The increment button. We tell it to produce an
                // `IncrementPressed` message when pressed
                Button::new(&mut self.start_button, Text::new("Start"))
                    .on_press(Message::StartPressed),
            )
            .push(
                // We show the value of the counter here
                match &self.elapsed {
                    Some(val) => Text::new(val.as_secs().to_string()).size(50),
                    None => Text::new("not started")
                }
            )
            .push(
                // The decrement button. We tell it to produce a
                // `DecrementPressed` message when pressed
                Button::new(&mut self.cancel_button, Text::new("Cancel"))
                    .on_press(Message::CancelPressed),
            )
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::StartPressed => {
                self.elapsed = Some(Duration::new(0, 0));
            }
            Message::CancelPressed => {
                self.elapsed = None;
            }
        }
    }
}

fn main() {
    Pomo::run(Settings::default())
}
