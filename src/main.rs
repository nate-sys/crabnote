use std::io::{stdin, stdout, Stdout, Write};
use termion::{color, cursor, event::Key, input::TermRead, raw::IntoRawMode, screen::*, style};
enum Component {
    H1(String),
    H2(String),
    P(String),
}
struct Page {
    components: Vec<Component>,
}
impl Page {
    fn draw(&self, stdout: &mut Stdout) {
        for (i, c) in self.components.iter().enumerate() {
            let current_position: (u16, u16) = (1, ((i + 1) * 2).try_into().unwrap());
            let mut text: String;
            match c {
                Component::H1(x) => {
                    write!(
                        stdout,
                        "{bg}{fg}{bolden}",
                        bg = color::Bg(color::Red),
                        fg = color::Fg(color::LightWhite),
                        bolden = style::Bold,
                    )
                    .unwrap();
                    text = String::from(x);
                    text.push_str(" ");
                    text.insert_str(0, " ");
                }
                Component::H2(x) => {
                    write!(
                        stdout,
                        "{bg}{fg}{bolden}",
                        bg = color::Bg(color::Blue),
                        fg = color::Fg(color::LightWhite),
                        bolden = style::Bold,
                    )
                    .unwrap();
                    text = String::from(x);
                    text.push_str(" ");
                    text.insert_str(0, " ");
                }
                Component::P(x) => {
                    write!(
                        stdout,
                        "{bg}{fg}",
                        bg = color::Bg(color::Black),
                        fg = color::Fg(color::White)
                    )
                    .unwrap();
                    text = String::from(x);
                }
            }

            write!(
                stdout,
                "{position}{this_text}{resetfg}{resetbold}",
                position = cursor::Goto(current_position.0, current_position.1),
                this_text = text,
                resetfg = color::Fg(color::Reset),
                resetbold = style::NoBold,
            )
            .unwrap();
        }
    }
}
fn main() {
    //init
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    stdout.flush().unwrap();

    let p = Page {
        components: vec![
            Component::H1(String::from("This is an h1")),
            Component::H2(String::from("This is an h2")),
            Component::P(String::from("This is a p")),
        ],
    };

    p.draw(&mut stdout);
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }
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
