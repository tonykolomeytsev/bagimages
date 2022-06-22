use std::{
    collections::BTreeMap,
    io::{stdout, Write},
};

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    QueueableCommand,
};

use crate::features::extract::extract::TopicState;
use crate::features::extract::view::View;

/// Because rustc output indent is 12
const INDENT_SIZE: usize = 12usize;

pub trait Indentable {
    fn indent(&self) -> String;
}

impl Indentable for &str {
    /// Add space indentation for the string if string len is lower than [INDENT_SIZE].
    ///
    /// # Example
    /// ```rust
    /// asserteq!("Done".indent(), "        Done".to_string())
    /// ```
    fn indent(&self) -> String {
        let len = self.len();
        let indent = if len <= INDENT_SIZE {
            INDENT_SIZE - len
        } else {
            0usize
        };
        format!("{:indent$}{}", "", &self, indent = indent)
    }
}

/// An interface for types that can be converted into a formatted color output.
pub trait Renderable {
    fn render(&self) -> String;
}

/// `Renderer` uses terminal for beautyful formatted color output.
///
/// Also see [Renderable] and its implementations.
pub struct Renderer();

impl Renderer {
    pub fn line<V>(&self, view: V)
    where
        V: Renderable,
    {
        let mut stdout = stdout();
        stdout.queue(cursor::MoveToPreviousLine(1u16)).unwrap();
        stdout
            .queue(terminal::Clear(ClearType::CurrentLine))
            .unwrap();
        stdout.write(view.render().as_bytes()).unwrap();
        stdout.write(b"\n").unwrap();
        stdout.flush().unwrap();
    }

    pub fn new_line(&self) {
        let mut stdout = stdout();
        stdout.write(b"\n").unwrap();
        stdout.flush().unwrap();
    }

    pub fn render(&self, states: &BTreeMap<u32, TopicState>, return_cursor: bool) {
        let mut stdout = stdout();
        stdout
            .queue(terminal::Clear(ClearType::FromCursorDown))
            .unwrap();

        for (_, state) in states {
            let view = if state.extracted == 0 {
                View::FoundTopic(state.name.clone())
            } else {
                View::ExtractedFromTopic(state.name.clone(), state.extracted)
            };
            stdout
                .queue(terminal::Clear(ClearType::CurrentLine))
                .unwrap();
            stdout.write(view.render().as_bytes()).unwrap();
            self.new_line();
        }

        if return_cursor {
            let lines_number = states.len() as u16;
            stdout
                .queue(cursor::MoveToPreviousLine(lines_number))
                .unwrap();
        } else {
            stdout.write(b"\n").unwrap();
        }
        stdout.flush().unwrap();
    }
}
