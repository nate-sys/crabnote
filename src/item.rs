use crate::component::Component;
use termion::event::Key;
#[allow(dead_code)]
#[derive(Clone)]
pub struct Item {
    component: Component,
    pub content: String,
    pub position: (u16, u16),
    pub lines: u16,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            component: Component::default(),
            content: String::from("Default content"),
            position: (1, 1),
            lines: 2,
        }
    }
}
impl Item {
    pub fn handle(&mut self, key: Key) {
        match key{
            Key::Char(character)=> {
                self.content.push(character);
            }
            _ => {}
        }
    }
    pub fn new_at_y(y: u16) -> Self {
        Item {
            component: Component::default(),
            content: String::from("Default content"),
            position: (1, y),
            lines: 2,
        }
    }
}
