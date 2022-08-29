use crate::component::Component;
use termion::{event::Key, color, style};
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
            content: String::from(""),
            position: (1, y),
            lines: 1,
            cursor_position: 0,
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
    pub fn handle(&mut self, key: Key) -> isize {
        match key{
            Key::Char(character) => {
                self.content.insert(self.cursor_position, character);
                self.cursor_position += 1;
            }
            Key::Left => self.go_left(),
            Key::Right => self.go_right(),
            Key::Up => {
               return -1; 
            }
            Key::Down => {
                return 1;
            }
            Key::Backspace => {
                if self.cursor_position > 0 {
                    self.content.remove(self.cursor_position -1);
                }
                self.go_left();
            }
            _ => {}
        }
        self.recalculate_lines();
        0
    }
    pub fn recalculate_lines(&mut self){
        self.lines = 1+self.content.len() as u16 / termion::terminal_size().unwrap().0;
    }
    pub fn parse_md(&mut self) -> (String, String) {
        let parsed_text: String;
        let formated_text: String;
        if self.content.starts_with("# ") {
            parsed_text = self.content.replace("# ", " ");
            self.adjustment = 0;
            formated_text = format!("{}{}{}", color::Fg(color::Green),style::Bold, style::Invert);
        }
        else if self.content.starts_with("## ") {
            parsed_text = self.content.replace("## ", "");
            self.adjustment = -2;
            formated_text = format!("{}{}", color::Fg(color::Green), style::Bold);
        }
        else if self.content.starts_with("### ") {
            parsed_text = self.content.replace("### ", "");
            self.adjustment = -3;
            formated_text = format!("{}", color::Fg(color::Green));
        }
        else if self.content.starts_with("- ") {
            parsed_text = self.content.replace("- ", "    • ");
            self.adjustment = 5;
            formated_text = format!("{}", color::Fg(color::Reset));
        }else if self.content.eq(""){
            parsed_text = String::from("...");
            self.adjustment = 1;

            formated_text = format!("{}{}",color::Fg(color::LightBlack) ,style::Italic);
        }
        else {
            formated_text = format!("{}", color::Fg(color::Reset));
            parsed_text = self.content.clone();
            self.adjustment = 1;
        }

        self.adjusted_cursor_position = (self.cursor_position as i32 + self.adjustment).try_into().unwrap_or(0);
        (parsed_text, formated_text)
    }
}
