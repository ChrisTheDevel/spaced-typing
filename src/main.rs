use std::io::{stdin, stdout, Stdin, Stdout, Write};

use termion::color::{self, Color, Fg, Red, Reset};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{clear, style};
use termion::{cursor, terminal_size};

const MOST_COMMON: &str = include_str!("../1000-most-common.txt");

struct TUI {
    stdout: RawTerminal<Stdout>,
    keys: Keys<Stdin>,
    width: usize,
    height: usize,
}

impl TUI {
    pub fn new(stdin: Stdin, stdout: RawTerminal<Stdout>) -> Self {
        let keys: Keys<Stdin> = stdin.keys();
        let (width, height) = terminal_size().unwrap();
        Self {
            stdout,
            keys,
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn clear(&mut self) -> AppResult<()> {
        let (width, height) = terminal_size()?;
        self.width = width as usize;
        self.height = height as usize;
        write!(self.stdout, "{}", clear::All)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn keys(&mut self) -> AppResult<Key> {
        Ok(self.keys.next().unwrap()?)
    }

    fn flush(&mut self) -> AppResult<()> {
        self.stdout.flush()?;
        Ok(())
    }

    /// renders the provided output string (with escape codes and all) over multiple lines such
    /// that one line is not wider than max_width.
    fn render(&mut self, output: &str, max_width: usize) -> AppResult<()> {
        struct FormatedString(String, usize);
        let mut lines: Vec<FormatedString> = Vec::new();
        let mut line = String::new();
        let mut line_width = 0;
        for char in output.chars() {
            if line_width > max_width {
                let (beg, end) = line.rsplit_once(' ').unwrap();
                let end_width = end.chars().map(|c| c.is_ascii()).count();
                lines.push(FormatedString(beg.to_string(), line_width - end_width));
                line_width = end_width;
                line = String::from(end.trim());
            }
            line.push(char);
            if char.is_ascii() {
                line_width += 1;
            }
        }
        // push the last line
        lines.push(FormatedString(line, line_width));

        let mid = self.width / 2;
        let x = mid - max_width / 2;
        let mut y = self.height / 2 - lines.len() / 2;
        for FormatedString(line, line_width) in lines {
            write!(
                self.stdout,
                "{}{line}",
                cursor::Goto(x as u16, y as u16),
                //cursor::Goto((x + line_width) as u16, y as u16)
            )?;
            y += 1;
        }
        Ok(())
    }
}

// struct holding program state, does nothing right now
pub struct SpacedTyping;
impl SpacedTyping {
    pub fn new() -> Self {
        Self
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let tui = TUI::new(stdin, stdout);

    let app = SpacedTyping::new();
    // TODO add error handling here
    run(app, tui).unwrap();
}

type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
enum AppError {
    TUIError(std::io::Error),
}

fn run(app: SpacedTyping, mut tui: TUI) -> AppResult<()> {
    let mut should_quit = false;
    // let wanted = "this is a long test sentence which hopefully spans multiple lines and will be nice to look at";
    // let typed = "this is a long test sentebce which hopefully bpans mulkiple lines and will be";
    let mut input = format!(
        "this is a test string with {}some ascii{} escape codes inserted",
        Fg(Red),
        Fg(Reset)
    );
    let max_width = 50;

    loop {
        // render
        tui.clear()?;
        tui.render(&input, max_width)?;
        tui.flush()?;

        // wait on event
        let key = tui.keys()?;

        // change state based on event
        match key {
            Key::Ctrl('c') | Key::Ctrl('q') => should_quit = true,
            Key::Char(c) => input.push(c),
            Key::Backspace => {
                input.pop();
            }
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
