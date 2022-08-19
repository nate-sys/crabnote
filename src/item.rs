use crate::component::Component;
use termion::{event::Key, color};
#[allow(dead_code)]
pub struct Item {
    component: Component,
    pub content: String,
    pub position: (u16, u16),
    pub lines: u16,
    cursor_position: usize,
    adjustment: i32,
    pub adjusted_cursor_position: usize,
}

impl Item {
    pub fn new(y: u16) -> Self {
        Item {
            component: Component::default(),
            content: String::from("Hello"),
            position: (1, y),
            lines: 1,
            cursor_position: 5,
            adjustment: 1,
            adjusted_cursor_position: 0,
        }
    }
    fn go_left(&mut self){
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }
    fn go_right(&mut self ){
        if self.cursor_position < self.content.len() {
            self.cursor_position += 1;
        }
    }
    pub fn handle(&mut self, key: Key) {
        match key{
            Key::Char(character) => {
                self.content.insert(self.cursor_position, character);
                self.cursor_position += 1;
            }
            Key::Left => self.go_left(),
            Key::Right => self.go_right(),
            Key::Backspace => {
                if self.cursor_position > 0 {
                    self.content.remove(self.cursor_position -1);
                }
                self.go_left();
            }
            _ => {}
        }
    }
    pub fn parse_md(&mut self) -> (String, String) {
        let parsed_text: String;
        let formated_text: String;
        if self.content.starts_with("# ") {
            parsed_text = self.content.replace("# ", "");
            self.adjustment = -1;
            formated_text = format!("{}", color::Fg(color::Green));
        }
        else if self.content.starts_with("## ") {
            parsed_text = self.content.replace("## ", "");
            self.adjustment = -2;
            formated_text = format!("{}", color::Fg(color::Yellow));
        }
        else if self.content.starts_with("### ") {
            parsed_text = self.content.replace("### ", "");
            self.adjustment = -3;
            formated_text = format!("{}", color::Fg(color::Red));
        }
        else if self.content.starts_with("- ") {
            parsed_text = self.content.replace("- ", "    • ");
            self.adjustment = 5;
            formated_text = format!("{}", color::Fg(color::Reset));
        }else{
            formated_text = format!("{}", color::Fg(color::Reset));
            parsed_text = self.content.clone();
            self.adjustment = 1;
        }

        self.adjusted_cursor_position = (self.cursor_position as i32 + self.adjustment).try_into().unwrap_or(0);
        return (parsed_text, formated_text)
    }
}
