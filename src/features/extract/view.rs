use std::fmt::Display;

use crossterm::style::Stylize;

use crate::features::renderer::Indentable;

pub enum View {
    RunningExport(Vec<String>),
    FoundTopic(String),
    ExtractedFromTopic(String, u32),
    // Info(String),
    IncompatibleTopicType(String, String, String),
    NoMessages(String, bool),
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
                "{} {} frame{} from topic {}",
                "Exported".indent().bold().green(),
                number,
                if *number == 1 { "" } else { "s" },
                name.clone().white().bold(),
            ),
            // View::Info(text) => format!("{} {}", "Info".indent().bold().yellow(), text),
            View::IncompatibleTopicType(topic, actual_type, expected_type) => {
                writeln!(
                    f,
                    "Topic {} has incompatible type `{}`, only `{}` is supported",
                    topic, actual_type, expected_type,
                )
            }
            View::NoMessages(topic, regex) => {
                if *regex {
                    writeln!(
                        f,
                        "{} found for topics with names matching regex `{}`",
                        "No messages".indent().bold().yellow(),
                        topic.clone().white().bold(),
                    )
                } else {
                    writeln!(
                        f,
                        "{} found for topic {}",
                        "No messages".indent().bold().yellow(),
                        topic.clone().white().bold(),
                    )?;
                    // maybe user forgot to specify regex option
                    if !regex && topic.chars().any(|ch| ch == '*' || ch == '\\') {
                        writeln!(
                            f,
                            "{:i$} maybe you forgot to specify the `-r` (`--regex`) option?",
                            "",
                            i = 12,
                        )?;
                    }
                    Ok(())
                }
            }
            View::Error(description) => {
                write!(f, "\n{} {}\n", "Error".indent().bold().red(), description)
            }
            View::Done => write!(f, "{}", "Done".indent().bold().green()),
        }
    }
}
