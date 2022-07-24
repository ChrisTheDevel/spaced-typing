use std::io::{stdin, stdout, Stdin, Stdout, Write};

use termion::color::{self, Color};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{clear, style};
use termion::{cursor, terminal_size};

const MOST_COMMON: &str = include_str!("../1000-most-common.txt");

// struct holding program state, does nothing right now
pub struct SpacedTyping;
impl SpacedTyping {
    pub fn new() -> Self {
        Self
    }
}

/// clears the screen
fn clear(stdout: &mut RawTerminal<Stdout>) -> AppResult<()> {
    write!(stdout, "{}", clear::All)?;
    stdout.flush()?;
    Ok(())
}

/// compares the actual with the wanted string and renders indicator of difference.
/// this function does not draw to screen
pub fn render_comparison(wanted: &str, actual: &str) -> String {
    let mut result = String::with_capacity(wanted.len());
    let mut wanted_chars = wanted.chars();
    let mut actual_chars = actual.chars();

    for (cw, ca) in wanted_chars.by_ref().zip(actual_chars.by_ref()) {
        if cw == ca {
            result.push(cw);
        } else {
            result.push_str(&color_char(ca, color::Red));
        }
    }
    let rest = if wanted.len() < actual.len() {
        let rest: String = actual_chars.collect();
        format!("{}{rest}{}", color::Fg(color::Red), color::Fg(color::Reset))
    } else {
        let rest: String = wanted_chars.collect();
        format!("{}{rest}{}", style::Faint, style::NoFaint)
    };
    result.push_str(&rest);
    result
}
fn color_char<C>(c: char, color: C) -> String
where
    C: Color,
{
    format!("{}{c}{}", color::Fg(color), color::Fg(color::Reset))
}

/// adds faint style to the text
fn with_faint(s: &str) -> String {
    format!("{}{}{}", style::Faint, s, style::NoFaint)
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let app = SpacedTyping::new();
    run(app, stdin, stdout);
}

type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
enum AppError {
    TUIError(std::io::Error),
}

fn run(app: SpacedTyping, stdin: Stdin, mut stdout: RawTerminal<Stdout>) -> AppResult<()> {
    // we want dynamic resizing of the terminal

    let mut should_quit = false;
    let mut keys: Keys<Stdin> = stdin.keys();
    let word = "This is a test sentence";
    let typed = "This is a best";

    loop {
        let (x, y) = terminal_size()?;
        // render
        clear(&mut stdout)?;
        let comp = render_comparison(&word, &typed);
        write!(
            stdout,
            "{}{comp}",
            cursor::Goto(x / 2 - (comp.len() / 2) as u16, 1),
        )?;
        stdout.flush()?;

        // wait on event
        let key: Key = keys.next().unwrap()?;
        // change state based on event
        match key {
            Key::Ctrl('c') | Key::Ctrl('q') => should_quit = true,
            _ => {}
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
