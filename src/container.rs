use crate::item::Item;
use std::io::{Stdout, Write};
use termion::cursor;

#[allow(dead_code)]
pub struct Container {
    pub items: Vec<Item>,
    pub inserting: bool,
    current_index: usize,
}

#[allow(dead_code)]
impl Container {
    pub fn new() -> Self {
        Container {
            items: vec![Item::default()],
            inserting: false,
            current_index: 0,
        }
    }
    pub fn length(&self) -> usize {
        self.items.len()
    }
    pub fn add_item(&mut self, i: Item) {
        self.items.push(i);
    }
    pub fn draw_to_buffer(&self, stdout: &mut Stdout) {
        for i in self.items.iter() {
            write!(
                stdout,
                "{position}{text}",
                position = cursor::Goto(i.position.0, i.position.1),
                text = i.content
            )
            .unwrap();
        }
    }
}
