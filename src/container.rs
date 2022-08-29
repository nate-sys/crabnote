use crate::item::Item;
use std::io::{Stdout, Write};
use termion::{cursor, style, event::Key};

pub struct Container {
    pub items: Vec<Item>,
    pub inserting: bool,
    current_index: isize,
}

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
    pub fn recalculate_all_lines(&mut self) {
        for i in self.items.iter_mut() {
            i.recalculate_lines();
        }
    }
    pub fn rearrange_items(&mut self) {
        let mut iter = self.items.iter_mut();
        if let Some(first) = iter.next(){
            first.position.1 = 1;
            let mut previous = first;
            for current in iter{
                current.position.1 = previous.position.1  + previous.lines + 1; 
                previous = current;
            }
        }
    }
    pub fn add_item(&mut self) {
        if self.items.is_empty() || self.current_index == self.length()-1 {
            self.items.push(Item::new((self.current_index + 1).try_into().unwrap_or(1)));
        }else{
            self.items.insert(self.current_index.try_into().unwrap_or(0)+1, Item::new(
                (self.current_index + 1).try_into().unwrap_or(1),
            ));
        }
        self.rearrange_items();
    }
    pub fn remove_item(&mut self) {
        if !self.items.is_empty() {
            self.items.remove(self.current_index.try_into().unwrap());
            self.current_index = self.current_index.min(self.length()-1).max(0);
        }else{ self.current_index = 0; }
        self.rearrange_items();
    }
    pub fn go(&mut self, i: isize) {
        self.recalculate_all_lines();
        self.current_index += i;
        self.current_index = self
            .current_index
            .clamp(0, (self.length() - 1).max(0))
            .max(0);
        self.recalculate_all_lines();
    }
    fn get_current_item(&self) -> &Item{
        self.items.get::<usize>(self.current_index.try_into().unwrap()).unwrap()
    }
    pub fn handle_insertion(&mut self, key: Key) {
        let mut movement = 0;
        if let Some(item) = self.items.get_mut::<usize>(self.current_index.try_into().unwrap()) {
            movement = item.handle(key);
        }
        self.go(movement);
    }
    pub fn draw_to_buffer(&mut self, stdout: &mut Stdout) {
        for (index, i) in self.items.iter_mut().enumerate() {
            if index == self.current_index.try_into().unwrap() && !self.inserting {
                write!(stdout, "{invert}", invert = style::Invert).unwrap();
            } else {
                write!(stdout, "{noInvert}", noInvert = style::NoInvert).unwrap();
            }
            let c = i.parse_md();
            write!(
                stdout,
                "{position}{style}{text}{reset}",
                position = cursor::Goto(i.position.0, i.position.1),
                style= c.1,
                text = c.0,
                reset = style::Reset,
            )
                .unwrap();
        }
        if !self.items.is_empty(){
            write!(
                stdout,
                "{}",
                cursor::Goto((self.get_current_item().adjusted_cursor_position).try_into().unwrap_or_default(), self.get_current_item().position.1)
            )
            .unwrap();
        }
    }
}
