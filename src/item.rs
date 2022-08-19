use crate::component::Component;
use termion::event::Key;
#[allow(dead_code)]
#[derive(Clone)]
pub struct Item {
    component: Component,
    pub content: String,
    pub position: (u16, u16),
    pub lines: u16,
    pub cursor_position: usize,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            component: Component::default(),
            content: String::from("Hello"),
            position: (1, 1),
            lines: 2,
            cursor_position: 5,
        }
    }
}
impl Item {
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
    pub fn new_at_y(y: u16) -> Self {
        Item {
            component: Component::default(),
            content: String::from("Hello"),
            position: (1, y),
            lines: 2,
            cursor_position: 5,
        }
    }
}
