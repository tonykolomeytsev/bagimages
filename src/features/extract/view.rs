use crossterm::style::Stylize;

use crate::features::renderer::{Indentable, Renderable};

pub enum View {
    FoundTopic(String),
    ExtractedFromTopic(String, u32),
    Done,
}

impl Renderable for View {
    fn render(&self) -> String {
        match &self {
            View::FoundTopic(name) => {
                format!(
                    "{} topic {}",
                    "Found".indent().bold().cyan(),
                    name.clone().white().bold(),
                )
            }
            View::ExtractedFromTopic(name, number) => format!(
                "{} {} frames from topic {}",
                "Extracted".indent().bold().green(),
                number,
                name.clone().white().bold(),
            ),
            View::Done => format!("{}", "Done".indent().bold().green()),
        }
    }
}
