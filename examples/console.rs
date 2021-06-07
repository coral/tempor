use std::io::{stdin, stdout, Write};
use std::time::Duration;
use tempor::Tapper;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut t = Tapper::new(4, Duration::from_secs(3)).unwrap();

    stdout.flush().unwrap();

    //detecting keydown events
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char(' ') => {
                match t.tap() {
                    Some(v) => {
                        println!("BPM: {}", v);
                    }
                    None => {}
                };
            }
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
