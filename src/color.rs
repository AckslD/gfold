use crate::types::Status;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn write_status(status: &Status, status_width: usize) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(match status {
        Status::Bare => Color::Red,
        Status::Clean => Color::Green,
        _ => Color::Yellow,
    })))?;
    write!(
        &mut stdout,
        "{:<status_width$}",
        match status {
            Status::Bare => "bare",
            Status::Clean => "clean",
            Status::Unclean => "unclean",
            Status::Unpushed => "unpushed",
        },
        status_width = status_width,
    )?;
    stdout.reset()
}

pub fn write_group_title(title: &str) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_bold(true))?;
    writeln!(&mut stdout, "{}", title)?;
    stdout.reset()
}
