use std::fmt::Display;

use crossterm::style::Stylize;

use crate::features::renderer::Indentable;

pub enum View {
    RunningExport(Vec<String>),
    FoundTopic(String),
    ExtractedFromTopic(String, u32),
    // Info(String),
    Error(String),
    Done,
}

impl Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            View::RunningExport(lines) => {
                writeln!(
                    f,
                    "{} bagimages v{} with following parameters:",
                    "Running".indent().bold().green(),
                    env!("CARGO_PKG_VERSION"),
                )?;
                for (i, line) in lines.iter().enumerate() {
                    if i != 0 {
                        writeln!(f)?;
                    }
                    write!(f, "{:i$} - {}", "", line, i = 12)?;
                }
                Ok(())
            }
            View::FoundTopic(name) => {
                write!(
                    f,
                    "{} topic {}",
                    "Found".indent().bold().cyan(),
                    name.clone().white().bold(),
                )
            }
            View::ExtractedFromTopic(name, number) => write!(
                f,
                "{} {} frames from topic {}",
                "Exported".indent().bold().green(),
                number,
                name.clone().white().bold(),
            ),
            // View::Info(text) => format!("{} {}", "Info".indent().bold().yellow(), text),
            View::Error(description) => {
                write!(f, "{} {}", "Error".indent().bold().red(), description)
            }
            View::Done => write!(f, "{}", "Done".indent().bold().green()),
        }
    }
}
