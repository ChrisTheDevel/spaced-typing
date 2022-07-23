use std::io::{stdin, stdout, Stdin, Stdout, Write};
use std::{collections::HashSet, time::Duration};

use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::RawTerminal;
use termion::{clear, input};
use termion::{cursor, terminal_size};
use termion::{raw::IntoRawMode, screen::AlternateScreen};

const MOST_COMMON: &str = include_str!("../1000-most-common.txt");

// struct holding program state
pub struct SpacedTyping;

pub struct Tui {
    stdin: Stdin,
    stdout: RawTerminal<Stdout>,
}

impl SpacedTyping {
    pub fn new() -> Self {
        Self
    }
}

impl Tui {
    pub fn new(stdin: Stdin, stdout: RawTerminal<Stdout>) -> Self {
        Self { stdin, stdout }
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let tui = Tui::new(stdin, stdout);
    let app = SpacedTyping::new();
    run(app, tui);
}

type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
enum AppError {
    TUIError(std::io::Error),
}

fn run(app: SpacedTyping, mut tui: Tui) -> AppResult<()> {
    let mut should_quit = false;
    let mut counter = 0;
    let mut keys: Keys<Stdin> = tui.stdin.keys();

    loop {
        let (x, y) = terminal_size()?;
        // render
        write!(
            tui.stdout,
            "{}{}counter: {counter}",
            clear::All,
            cursor::Goto(x / 2, y / 2),
        )?;
        tui.stdout.flush()?;
        // wait on event
        let key: Key = keys.next().unwrap()?;
        // change state based on event
        match key {
            Key::Ctrl('c') | Key::Ctrl('q') => should_quit = true,
            _ => counter += 1,
        }

        if should_quit {
            break;
        }
    }
    Ok(())
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::TUIError(err)
    }
}
