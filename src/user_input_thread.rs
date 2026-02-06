use std::{io::stdin, sync::mpsc, thread};

pub enum UserInput {
    PlayNote(String),
    Quit,
}

pub struct UserInputThread {
    sender: mpsc::Sender<UserInput>,
}

impl UserInputThread {
    pub fn new(sender: mpsc::Sender<UserInput>) -> Self {
        Self { sender }
    }

    pub fn start(self) {
        thread::spawn(move || -> anyhow::Result<()> {
            loop {
                let mut user_input = String::new();

                stdin().read_line(&mut user_input)?;

                if user_input.is_empty() {
                    continue;
                }

                let args: Vec<&str> = user_input.trim().split(' ').collect();

                match args[0] {
                    "quit" | "q" => {
                        self.sender.send(UserInput::Quit)?;
                    }
                    note => {
                        self.sender.send(UserInput::PlayNote(note.to_string()))?;
                    }
                }
            }
        });
    }
}
