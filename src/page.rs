use std::io::{Stdout, Write};
use termion::{color, cursor, style};
pub struct Page {
    pub components: Vec<String>,
}
impl Page {
    pub fn new_line(&mut self) {
        self.components.push(String::new());
    }
    pub fn draw(&self, i: usize, stdout: &mut Stdout) {
        let current_position: (u16, u16) = (1, ((i + 1) * 2).try_into().unwrap());
        let mut text: String;
        let c = &self.components[i];
        match c {
            s if s.starts_with("# ") => {
                write!(
                    stdout,
                    "{bg}{fg}{bolden}",
                    bg = color::Bg(color::Red),
                    fg = color::Fg(color::LightWhite),
                    bolden = style::Bold,
                )
                .unwrap();
                text = String::from(s);
                text = text.replacen("# ", " ", 1);
                text.push_str(" ");
            }
            s if s.starts_with("## ") => {
                write!(
                    stdout,
                    "{bg}{fg}{bolden}",
                    bg = color::Bg(color::Blue),
                    fg = color::Fg(color::LightWhite),
                    bolden = style::Bold,
                )
                .unwrap();
                text = String::from(s);
                text = text.replacen("## ", " ", 1);
                text.push_str(" ");
            }
            _ => {
                write!(
                    stdout,
                    "{bg}{fg}",
                    bg = color::Bg(color::Black),
                    fg = color::Fg(color::White)
                )
                .unwrap();
                text = String::from(c);
            }
        }

        write!(
            stdout,
            "{position}{this_text}{resetbg}{resetfg}{resetbold}",
            position = cursor::Goto(current_position.0, current_position.1),
            this_text = text,
            resetbg = color::Bg(color::Reset),
            resetfg = color::Fg(color::Reset),
            resetbold = style::NoBold,
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}
