use std::io::{self, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::color;
use termion::cursor;

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Size {
    pub width : u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size, 
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2), // For the two status lines
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn cursor_position(position: &Position) {
        let Position {x, y} = position;
        let x = *x as u16;
        let y = *y as u16;
        print!("{}", cursor::Goto(x.saturating_add(1), y.saturating_add(1)));
    }

    pub fn centered(s: &str, padding_c: char) -> String {
        let mut result = s.to_string();
        let width = termion::terminal_size().unwrap().0 as usize;

        let len = result.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = padding_c.to_string().repeat(padding);

        result = format!("{spaces}{result}{spaces}");
        result.truncate(width);
        result
    }

    pub fn repeated(s: char) -> String {
        let width = termion::terminal_size().unwrap().0 as usize;
        s.to_string().repeat(width)
    }

    pub fn hide_cursor() {
        print!("{}", cursor::Hide);
    }

    pub fn show_cursor() {
        print!("{}", cursor::Show);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color))
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color))
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    pub fn flush() -> Result<(), io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}