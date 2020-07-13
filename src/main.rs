use iced::text_input;
use iced::{
    button, executor, time, Application, Button, Column, Command, Container, Element, Length, Row,
    Settings, Subscription, Text, TextInput,
};
use std::io::Cursor;
use std::time::{Duration, Instant};

struct Pomo {
    remaining: Duration,
    state: PomoState,
    length: Duration,

    // The local state of the button
    toggle_button: button::State,
    toggle_button_text: String,
    pomo_length_input: text_input::State,
    pomo_length_input_val: String,
}

enum PomoState {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePressed,
    Tick(Instant),
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
                remaining: Duration::from_secs(MINUTE * 2),
                state: PomoState::Idle,
                length: Duration::from_secs(MINUTE * 2),
                toggle_button: button::State::new(),
                toggle_button_text: String::from("Start"),
                pomo_length_input: text_input::State::new(),
                pomo_length_input_val: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pomodoro and pause")
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let input = TextInput::new(
            &mut self.pomo_length_input,
            "pomo length",
            &self.pomo_length_input_val,
            Message::PomoLengthChanged,
        )
        .size(25)
        .width(Length::Units(30))
        .padding(5);
        let seconds = self.remaining.as_secs();
        let remaining = Text::new(format!(
            "{:0>2}:{:0>2}",
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
        ))
        .size(40);
        let buttons_row = Row::new().push(
            Button::new(&mut self.toggle_button, Text::new(&self.toggle_button_text))
                .on_press(Message::TogglePressed),
        );
        let content = Column::new().push(remaining).push(buttons_row).push(input);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TogglePressed => match self.state {
                PomoState::Idle => {
                    self.state = PomoState::Ticking {
                        last_tick: Instant::now(),
                    };
                    self.toggle_button_text = String::from("Cancel");
                }
                PomoState::Ticking { .. } => {
                    self.remaining = self.length;
                    self.state = PomoState::Idle;
                    self.toggle_button_text = String::from("Start");
                }
            },
            Message::Tick(now) => if let PomoState::Ticking { last_tick } = &mut self.state {
                self.remaining -= now - *last_tick;
                *last_tick = now;

                if self.remaining.as_secs() == 0 {
                    self.remaining = self.length;
                    self.state = PomoState::Idle;
                    play_pomo_done();
                }
            },
            Message::PomoLengthChanged(length) => {
                self.pomo_length_input_val = length.clone();
                if let Ok(val) = length.parse::<u64>() {
                    self.length = Duration::from_secs(val * MINUTE);
                    self.remaining = self.length;
                };
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            PomoState::Idle => Subscription::none(),
            PomoState::Ticking { .. } => time::every(Duration::from_millis(10)).map(Message::Tick),
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
    Pomo::run(Settings {
        window: iced::window::Settings {
            resizable: false,
            size: (150, 100),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}
