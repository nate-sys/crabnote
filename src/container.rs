use crate::item::Item;
use std::io::{Stdout, Write};
use termion::{cursor, style, event::Key};

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
            items: Vec::<Item>::new(),
            inserting: false,
            current_index: 0,
        }
    }
    pub fn length(&self) -> isize {
        self.items.len().try_into().unwrap_or_default()
    }
    fn rearrange_items(&mut self) {
        let mut iter = self.items.iter_mut();
        if let Some(first) = iter.next(){
            let mut prev = first;
            while let Some(elem) = iter.next(){
                elem.position.1 = prev.position.1  + prev.lines; 
                prev = elem;
            }
        }
    }
    pub fn add_item(&mut self) {
        self.items.push(Item::new_at_y(
            (self.current_index + 1).try_into().unwrap_or(1),
        ));
        self.rearrange_items();
    }
    pub fn remove_item(&mut self) {
        if !self.items.is_empty() {
            self.items.remove(self.current_index.try_into().unwrap());
            if self.current_index == self.length() {
                self.current_index -= 1;
            }
        }
        self.rearrange_items();
    }
    pub fn go(&mut self, i: isize) {
        self.current_index += i;
        self.current_index = self
            .current_index
            .clamp(0, (self.length() - 1).max(0))
            .max(0);
    }
    fn get_current_item(&self) -> &Item{
        self.items.get::<usize>(self.current_index.try_into().unwrap()).unwrap()
    }
    pub fn handle_insertion(&mut self, key: Key) {
        if let Some(item) = self.items.get_mut::<usize>(self.current_index.try_into().unwrap()) {
            item.handle(key);
        }
    }
    pub fn draw_to_buffer(&self, stdout: &mut Stdout) {
        for (index, i) in self.items.iter().enumerate() {
            if index == self.current_index.try_into().unwrap() && !self.inserting {
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
            write!(
                stdout,
                "{}",
                cursor::Goto((self.get_current_item().content.len() + 1).try_into().unwrap(), self.get_current_item().position.1)
                  ).unwrap();
        }
    }
}
