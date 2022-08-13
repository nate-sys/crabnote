use std::fs::File;
use std::io::{BufRead, BufReader, Stdout, Write};
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
                    bg = color::Bg(color::Green),
                    fg = color::Fg(color::Black),
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
                    bg = color::Bg(color::Reset),
                    fg = color::Fg(color::Green),
                    bolden = style::Bold,
                )
                .unwrap();
                text = String::from(s);
                text = text.replacen("## ", "", 1);
                text.push_str(" ");
            }
            s if s.starts_with("### ") => {
                write!(
                    stdout,
                    "{bg}{fg}",
                    bg = color::Bg(color::Reset),
                    fg = color::Fg(color::Green),
                )
                .unwrap();
                text = String::from(s);
                text = text.replacen("### ", "", 1);
                text.push_str(" ");
            }
            s if s.starts_with("* ") => {
                write!(stdout, "{fg}", fg = color::Fg(color::Reset),).unwrap();
                text = String::from(s);
                text = text.replacen("* ", "  • ", 1);
            }
            s if s.starts_with("- ") => {
                write!(stdout, "{fg}", fg = color::Fg(color::Reset),).unwrap();
                text = String::from(s);
                text = text.replacen("- ", "  ◦ ", 1);
            }
            _ => {
                write!(
                    stdout,
                    "{bg}{fg}",
                    bg = color::Bg(color::Reset),
                    fg = color::Fg(color::Reset)
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
    pub fn read(&mut self) {
        let file = File::open("./note.md").expect("unable to open file");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let l = line.expect("unable to read line");
            self.components.push(l);
        }
    }
    pub fn save(&self) {
        let data = self.components.join("\n");
        if let Ok(mut f) = File::create("./note.md") {
            f.write_all(data.as_bytes()).unwrap();
        }
    }
}
