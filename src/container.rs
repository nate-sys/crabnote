use crate::item::Item;
use std::io::{Stdout, Write};
use termion::{cursor, style};

#[allow(dead_code)]
pub struct Container {
    pub items: Vec<Item>,
    pub inserting: bool,
    current_index: isize,
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
    fn rearrange_items(&mut self) {
        self.items
            .iter_mut()
            .enumerate()
            .for_each(|(ind, it)| it.position.1 = (ind + 1).try_into().unwrap());
    }
    pub fn add_item(&mut self) {
        self.items.push(Item::new_at_y(
            (self.current_index + 1).try_into().unwrap_or(0),
        ));
        self.rearrange_items();
    }
    pub fn remove_item(&mut self) {
        self.items.remove(self.current_index.try_into().unwrap());
        self.items
            .iter_mut()
            .enumerate()
            .for_each(|(ind, mut item)| item.position.1 = ind.try_into().unwrap_or(0));
        self.rearrange_items();
    }
    pub fn go(&mut self, i: isize) {
        self.current_index += i;
        self.current_index = self
            .current_index
            .clamp(0, (self.length() - 1).max(0).try_into().unwrap());
    }
    pub fn draw_to_buffer(&self, stdout: &mut Stdout) {
        for (index, i) in self.items.iter().enumerate() {
            if index == self.current_index.try_into().unwrap() {
                write!(stdout, "{invert}", invert = style::Invert).unwrap();
            } else {
                write!(stdout, "{noInvert}", noInvert = style::NoInvert).unwrap();
            }
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
