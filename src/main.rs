mod component;
mod container;
mod item;
use std::io::{stdin, stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::*};
fn main() {
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut item_list = container::Container::new();
    write!(stdout, "{}", ToAlternateScreen).unwrap();
    for c in stdin.keys() {
        if item_list.inserting {
            match c.unwrap() {
                Key::Esc => item_list.inserting = false,
                _ => {}
            }
        } else {
            match c.unwrap() {
                Key::Char('q') => break,
                Key::Char('i') => item_list.inserting = true,
                Key::Char('a') => item_list.add_item(item::Item::new_at_y(
                    (item_list.length() + 1).try_into().unwrap_or(0),
                )),
                _ => {}
            }
        }
        write!(stdout, "{}", termion::clear::All).unwrap();
        item_list.draw_to_buffer(&mut stdout);
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", ToMainScreen).unwrap();
    stdout.flush().unwrap();
}
