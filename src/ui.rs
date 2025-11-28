use std::{
    fmt::{self, Write as FmtWrite},
    io::{self, Write},
    mem,
};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Color, Print, Stylize},
    terminal::{Clear, ClearType},
};

use crate::pitch::info::get_pitch_info;

const TEXT_TO_PRINT_CAPACITY: usize = 128;

fn get_color_from_cent(cent: f64) -> Color {
    let percentage = 1.0 - cent.abs() * 2.0;

    let (mut r, mut g, b) = (255.0, 255.0, 255.0 * percentage);

    if cent < 0.0 {
        g *= percentage;
    } else if cent > 0.0 {
        r *= percentage;
    }

    Color::Rgb {
        r: r.round() as u8,
        g: g.round() as u8,
        b: b.round() as u8,
    }
}

pub struct Ui {
    text_to_print: String,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            text_to_print: String::with_capacity(TEXT_TO_PRINT_CAPACITY),
        }
    }

    fn write_text_to_print(&mut self, frequency: f32) -> fmt::Result {
        let pitch_info = get_pitch_info(frequency);

        writeln!(self.text_to_print, "Pitch: {:.1} hertz", frequency)?;

        let cent = pitch_info.cent;

        let color = get_color_from_cent(cent);

        write!(self.text_to_print, "\n\n")?;
        if cent > 0.0 {
            writeln!(
                self.text_to_print,
                "{}",
                format!("{:+.1}", cent * 100.0).with(color)
            )?;
        } else {
            writeln!(self.text_to_print)?;
        }

        let mut pitch_info_note = pitch_info.note.to_string();
        if cent.abs() < 0.1 {
            pitch_info_note = pitch_info_note.on_green().to_string();
        }

        let pitch_hint = if cent > 0.1 {
            "↓"
        } else if cent < -0.1 {
            "↑"
        } else {
            ""
        };

        writeln!(self.text_to_print, "\n{} {}\n", pitch_info_note, pitch_hint)?;

        if cent < 0.0 {
            writeln!(
                self.text_to_print,
                "{}",
                format!("{:.1}", cent * 100.0).with(color)
            )?;
        }

        Ok(())
    }

    pub fn render(&mut self, frequency: f32) -> io::Result<()> {
        self.write_text_to_print(frequency)
            .expect("Should be able to write to string");

        let text_to_print_owned = mem::replace(
            &mut self.text_to_print,
            String::with_capacity(TEXT_TO_PRINT_CAPACITY),
        );

        io::stdout()
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?
            .queue(Print(text_to_print_owned))?
            .flush()?;

        Ok(())
    }
}
