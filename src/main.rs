use std::io::{stdin, stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::*};
mod page;
use page::Page;
fn main() {
    //init
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    stdout.flush().unwrap();

    let mut p = Page {
        components: vec![String::new()],
    };
    for (i, _) in p.components.iter().enumerate() {
        p.draw(i, &mut stdout);
    }

    let mut current_index = 0;
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(c) => {
                if c != '\n' {
                    if let Some(current_line) = p.components.iter_mut().nth(current_index) {
                        current_line.push(c);
                    }
                } else {
                    p.new_line();
                    current_index += 1;
                }
            }
            Key::Backspace => {
                if let Some(current_line) = p.components.iter_mut().nth(current_index) {
                    if let Some(_) = current_line.pop() {
                        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
                    } else {
                        if current_index > 0 {
                            current_index -= 1;
                        }
                    }
                }
            }
            _ => break,
        }
        p.draw(current_index, &mut stdout);
    }
    //try to reset on exit
    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All,
        termion::cursor::Show,
    )
    .unwrap();
}
