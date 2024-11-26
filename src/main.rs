mod cli;

use clap::Parser;
use cli::Args;
use colored::Colorize;
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args = Args::parse();

    let status_text = format!(
        "Searching Reddit /r/anime for {} discussion threads...",
        args.name.green().bold()
    )
    .blue().to_string();
    let mut spinner = Spinner::new(Spinners::Dots, status_text.into());
    sleep(Duration::from_secs(3));
    spinner.stop();
}
