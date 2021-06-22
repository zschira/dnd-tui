use termion::event::Key;
use crate::select_events::SelectResponse;
use log::info;

pub struct InputBuffer {
    pub value: String,
    pub cursor: usize,
}

impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer {
            value: String::with_capacity(32),
            cursor: 0
        }
    }

    pub fn key_stroke(&mut self, key: Key) -> SelectResponse {
        match key {
            Key::Char(c) => { 
                if c != '\n' {
                    self.value.insert(self.cursor, c);
                    self.cursor += 1;
                    SelectResponse::None
                } else {
                    SelectResponse::Search{name: "".to_string(), value: self.value.clone()}
                }
            },
            Key::Backspace => {
                if self.cursor > 0 {
                    self.value.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
                SelectResponse::None
            },
            Key::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                SelectResponse::None
            },
            Key::Right => {
                if self.cursor < self.value.len() {
                    self.cursor += 1;
                }
                SelectResponse::None
            },
            Key::Delete => {
                if self.cursor < self.value.len() {
                    self.value.remove(self.cursor + 1);
                }
                SelectResponse::None
            },
            Key::End => { 
                self.cursor = self.value.len() + 1;
                SelectResponse::None
            },
            Key::Home => {
                self.cursor = 0;
                SelectResponse::None
            },
            _ => {
                SelectResponse::None
            }
        }
    }
}
