use iced::{button, executor, time, Application, Button, Column, Command, Element, Settings, Subscription, Text, TextInput};
use iced::text_input;
use std::time::{Duration, Instant};
use std::io::Cursor;

struct Pomo {
    remaining: Duration,
    state: PomoState,
    length: Duration,

    // The local state of the two buttons
    cancel_button: button::State,
    start_button: button::State,
    pomo_length_input: text_input::State,
    pomo_length_input_val: String,
}

enum PomoState {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Debug, Clone)]
pub enum Message {
    StartPressed,
    Tick(Instant),
    CancelPressed,
    PomoLengthChanged(String),
}

const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;

impl Application for Pomo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Pomo, Command<Message>) {
        (
            Pomo {
                remaining: Duration::from_secs(MINUTE * 1),
                state: PomoState::Idle,
                length: Duration::from_secs(MINUTE * 1),
                cancel_button: button::State::new(),
                start_button: button::State::new(),
                pomo_length_input: text_input::State::new(),
                pomo_length_input_val: String::new(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let input = TextInput::new(
            &mut self.pomo_length_input,
            "pomo length",
            &self.pomo_length_input_val,
            Message::PomoLengthChanged,
        ).padding(15).size(10);
        let seconds = self.remaining.as_secs();
        let remaining = Text::new(format!(
            "{:0>2}:{:0>2}",
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
        ));
        Column::new()
            .push(
                Button::new(&mut self.start_button, Text::new("Start"))
                    .on_press(Message::StartPressed),
            )
            .push(remaining)
            .push(
                Button::new(&mut self.cancel_button, Text::new("Cancel"))
                    .on_press(Message::CancelPressed),
            )
            .push(input)
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
                        self.remaining -= now - *last_tick;
                        *last_tick = now;

                        if self.remaining.as_secs()  == 0 {
                            self.state = PomoState::Idle;
                            play_pomo_done();
                        }
                    },
                    _ => {}
                }
            }
            Message::CancelPressed => {
                self.remaining = self.length;
                self.state = PomoState::Idle;
            }
            Message::PomoLengthChanged(length) => {
                self.pomo_length_input_val = length.clone();
                self.length = Duration::from_secs(length.parse::<u64>().unwrap() * MINUTE);
                self.remaining = self.length;
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            PomoState::Idle => Subscription::none(),
            PomoState::Ticking { .. } => {
                time::every(Duration::from_millis(10)).map(Message::Tick)
            },
        }
    }
}

fn play_pomo_done() {
    use rodio::Source;

    let device = rodio::default_output_device().unwrap();

    let buf = Cursor::new(include_bytes!("../assets/ring.mp3").to_vec());
    let source = rodio::Decoder::new(buf).unwrap();
    rodio::play_raw(&device, source.convert_samples());
}

fn main() {
    Pomo::run(Settings::default())
}
