use iced::{button, executor, time, Application, Button, Column, Command, Element, Settings, Subscription, Text};
use std::time::{Duration, Instant};

struct Pomo {
    elapsed: Duration,
    state: PomoState,

    // The local state of the two buttons
    cancel_button: button::State,
    start_button: button::State,
}

enum PomoState {
    Idle,
    Ticking { last_tick: Instant }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    StartPressed,
    Tick(Instant),
    CancelPressed,
}


impl Application for Pomo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Pomo, Command<Message>) {
        (
            Pomo {
                elapsed: Duration::default(),
                state: PomoState::Idle,
                cancel_button: button::State::new(),
                start_button: button::State::new(),
            },
            Command::none()
        )
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
                Text::new(&self.elapsed.as_secs().to_string()).size(50),
            )
            .push(
                // The decrement button. We tell it to produce a
                // `DecrementPressed` message when pressed
                Button::new(&mut self.cancel_button, Text::new("Cancel"))
                    .on_press(Message::CancelPressed),
            )
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::StartPressed => {
                match self.state {
                    PomoState::Idle => {
                        self.state = PomoState::Ticking {
                            last_tick: Instant::now(),
                        }
                    },
                    _ => {}
                }
            },
            Message::Tick(now) => {
                match &mut self.state {
                    PomoState::Ticking { last_tick } => {
                        self.elapsed += now - *last_tick;
                        *last_tick = now;
                    },
                    _ => {}
                }
            }
            Message::CancelPressed => {
                self.elapsed = Duration::default();
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            PomoState::Idle => Subscription::none(),
            PomoState::Ticking { .. } => {
                time::every(Duration::from_millis(10)).map(Message::Tick)
            }
        }
    }
}

fn main() {
    Pomo::run(Settings::default())
}
