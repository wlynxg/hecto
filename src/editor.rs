use crossterm::event::Event::Key;
use crossterm::event::KeyCode::Char;
use crossterm::event::read;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        return Editor {};
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);

                    match event.code {
                        Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        disable_raw_mode().unwrap();
    }
}