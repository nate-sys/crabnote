mod component;
mod container;
mod item;
use std::io::{stdin, stdout, Write};
use termion::{cursor, event::Key, input::TermRead, raw::IntoRawMode, screen::*};
fn main() -> Result<(), &'static str> {
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode()?);
    let mut item_list = container::Container::new();
    item_list.add_item();
    item_list.inserting = true;
    write!(
        stdout,
        "{toAlt}{hideCursor}{gotop}",
        toAlt = ToAlternateScreen,
        hideCursor = cursor::Hide,
        gotop = cursor::Goto(1,1)
    )?;
    for c in stdin.keys() {
        if item_list.inserting {
            if c.as_ref()? == &Key::Esc {
                item_list.inserting = false
            }
            else if c.as_ref().unwrap() == &Key::Char('\n'){
                //todo
                item_list.add_item();
                item_list.go(1);
            }
            else{
                item_list.handle_insertion(c.unwrap());
            }
        }else {
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
        item_list.recalculate_all_lines();
        item_list.rearrange_items();
        if item_list.inserting{
            write!(stdout, "{}{}",cursor::Show, cursor::BlinkingBlock).unwrap();
        }else{
            write!(stdout, "{}", cursor::Hide).unwrap();
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
    ).unwrap();
    stdout.flush().unwrap();
}
