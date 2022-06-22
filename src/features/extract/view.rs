use crossterm::style::Stylize;

use crate::features::renderer::{Indentable, Renderable};

pub enum View {
    RunningExport(Vec<String>),
    FoundTopic(String),
    ExtractedFromTopic(String, u32),
    // Info(String),
    Done,
}

impl Renderable for View {
    fn render(&self) -> String {
        match &self {
            View::RunningExport(lines) => {
                let lines = lines
                    .iter()
                    .map(|line| format!("{:i$} - {}", "", line, i = 12))
                    .collect::<Vec<String>>()
                    .join("\n");
                format!(
                    "{} bagimages v{} with following parameters:\n{}",
                    "Running".indent().bold().green(),
                    env!("CARGO_PKG_VERSION"),
                    lines,
                )
            }
            View::FoundTopic(name) => {
                format!(
                    "{} topic {}",
                    "Found".indent().bold().cyan(),
                    name.clone().white().bold(),
                )
            }
            View::ExtractedFromTopic(name, number) => format!(
                "{} {} frames from topic {}",
                "Exported".indent().bold().green(),
                number,
                name.clone().white().bold(),
            ),
            // View::Info(text) => format!("{} {}", "Info".indent().bold().yellow(), text),
            View::Done => format!("{}", "Done".indent().bold().green()),
        }
    }
}
