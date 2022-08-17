mod component;
mod container;
mod item;
use std::io::{stdin, stdout, Write};
use termion::{cursor, event::Key, input::TermRead, raw::IntoRawMode, screen::*};
fn main() {
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut item_list = container::Container::new();
    write!(
        stdout,
        "{toAlt}{hideCursor}",
        toAlt = ToAlternateScreen,
        hideCursor = cursor::Hide
    )
    .unwrap();
    for c in stdin.keys() {
        if item_list.inserting {
            if c.unwrap() == Key::Esc {
                item_list.inserting = false
            }
        } else {
            match c.unwrap() {
                Key::Char('q') => break,
                Key::Char('i') => item_list.inserting = true,
                Key::Char('a') => item_list.add_item(),
                Key::Char('d') => item_list.remove_item(),
                Key::Char('j') => item_list.go(1),
                Key::Char('k') => item_list.go(-1),
                _ => {}
            }
        }
        write!(stdout, "{}", termion::clear::All).unwrap();
        item_list.draw_to_buffer(&mut stdout);
        stdout.flush().unwrap();
    }
    write!(
        stdout,
        "{toMain}{cursorShow}",
        toMain = ToMainScreen,
        cursorShow = cursor::Show
    )
    .unwrap();
    stdout.flush().unwrap();
}
