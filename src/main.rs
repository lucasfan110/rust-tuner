use std::{
    io::stdout,
    process,
    str::FromStr,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossterm::{
    ExecutableCommand,
    terminal::{Clear, ClearType},
};
use pitch::{detect::PitchFrequencyDetector, info::Note, player::PitchPlayer};
use ui::Ui;
use user_input_thread::{UserInput, UserInputThread};

const UI_RENDERS_PER_SEC: u64 = 10;
const MILLISECONDS_PER_RENDER: u64 = 1_000 / UI_RENDERS_PER_SEC;
const UI_RENDER_TICK_TIME: Duration = Duration::from_millis(MILLISECONDS_PER_RENDER);

mod pitch;
mod ui;
mod user_input_thread;

fn handle_user_input(user_input: &UserInput, pitch_player: &mut PitchPlayer) {
    use UserInput::*;

    match user_input {
        Quit => process::exit(0),
        PlayNote(note) => {
            let note = match Note::from_str(note) {
                Ok(note) => note,
                Err(e) => {
                    eprintln!("Invalid note or command! ({})", e);
                    return;
                }
            };

            let pitch = note.get_pitch();

            pitch_player
                .play_pitch(pitch)
                .expect("Should be able to play pitch");
        }
    }
}

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Should have a default input device");
    let config = device
        .default_input_config()
        .expect("Should have a default input config")
        .config();

    let (sender, receiver) = mpsc::channel::<UserInput>();

    let user_input_thread = UserInputThread::new(sender);

    let mut pitch_frequency_detector = PitchFrequencyDetector::new(config.clone());
    let mut ui = Ui::new();

    let mut pitch_player = PitchPlayer::new().expect("Should be able to construct pitch player");

    let mut last_render_timestamp = Instant::now();

    let stream = device
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if last_render_timestamp.elapsed() < UI_RENDER_TICK_TIME {
                    return;
                }

                let frequency = pitch_frequency_detector.find_pitch_frequency(data);

                if let Some(frequency) = frequency {
                    ui.render(frequency).expect("Should be able to render UI");
                }

                last_render_timestamp = Instant::now();
            },
            move |_| {},
            None,
        )
        .expect("Should be able to build input stream");

    stream.play().expect("Should be able to play input stream");

    stdout()
        .execute(Clear(ClearType::All))
        .expect("Should be able to clear screen");

    println!("Listening for a note...");

    user_input_thread.start();

    loop {
        thread::sleep(Duration::from_millis(1));

        if let Ok(message) = receiver.try_recv() {
            handle_user_input(&message, &mut pitch_player);

            stdout()
                .execute(Clear(ClearType::All))
                .expect("Should be able to clear screen");

            println!("Listening for a note...");
        }
    }
}
