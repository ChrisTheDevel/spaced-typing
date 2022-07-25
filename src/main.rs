use std::io::{stdin, stdout, Stdin, Stdout, Write};

use termion::color::{self, Color};
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
    width: u16,
    height: u16,
}

impl TUI {
    pub fn new(stdin: Stdin, stdout: RawTerminal<Stdout>) -> Self {
        let keys: Keys<Stdin> = stdin.keys();
        let (width, height) = terminal_size().unwrap();
        Self {
            stdout,
            keys,
            width,
            height,
        }
    }

    pub fn clear(&mut self) -> AppResult<()> {
        let (width, height) = terminal_size()?;
        self.width = width;
        self.height = height;
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

    fn render(&mut self, wanted: &str, actual: &str, max_width: u16) -> AppResult<()> {
        let mut wchars = wanted.chars();
        let mut achars = actual.chars();

        let mut lines: Vec<String> = Vec::new();
        let mut line = String::new();
        let mut line_width = 0;

        for (cw, ca) in wchars.by_ref().zip(achars.by_ref()) {
            if line_width > max_width {
                // when the line has gone wider than max, we split away the last word and put it on the next line
                if let Some((beg, rest)) = line.rsplit_once(' ') {
                    let rest: String = rest.chars().filter(|c| c.is_ascii()).collect();
                    let rest_len = rest.len() as u16;
                    lines.push(beg.to_string());
                    line = String::from(rest);
                    line_width = rest_len;
                } else {
                    lines.push(line);
                    line = String::new();
                    line_width = 0;
                }
            }
            if cw == ca {
                line.push(cw);
            } else {
                line.push_str(&format!(
                    "{}{ca}{}",
                    color::Fg(color::Red),
                    color::Fg(color::Reset)
                ));
            }
            line_width += 1;
        }

        let (format_red, rest) = if wanted.len() > actual.len() {
            (false, wchars)
        } else {
            (true, achars)
        };

        if format_red {
            line.push_str(&color::Fg(color::Red).to_string());
        } else {
            line.push_str(&style::Faint.to_string());
        }

        for c in rest {
            if line_width > max_width {
                // when the line has gone wider than max, we split away the last word and put it on the next line
                if let Some((beg, rest)) = line.rsplit_once(' ') {
                    let rest: String = rest.chars().filter(|c| c.is_ascii()).collect();
                    let rest_len = rest.len() as u16;
                    lines.push(beg.to_string());
                    line = String::from(rest);
                    line_width = rest_len;
                } else {
                    lines.push(line);
                    line = String::new();
                    line_width = 0;
                }
            }
            line.push(c);
            line_width += 1;
        }

        if format_red {
            line.push_str(&color::Fg(color::Reset).to_string());
        } else {
            line.push_str(&style::NoFaint.to_string());
        }
        lines.push(line);

        let mid = self.width / 2 as u16;
        let mut y = self.height / 2 - (lines.len() / 2) as u16;
        for s in lines {
            write!(
                self.stdout,
                "{}{s}",
                cursor::Goto(mid - (max_width / 2) as u16, y),
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
    /// TODO add error handling here
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
    let wanted = "this is a long test sentence which hopefully spans multiple lines and will be nice to look at";
    let actual = "this is a long test sentbnce which hopefclly spans multiple lines and will be nice to look at but too long";

    loop {
        // render
        tui.clear()?;
        tui.render(wanted, actual, 50)?;
        tui.flush()?;

        // change state based on event
        let key = tui.keys()?;
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
