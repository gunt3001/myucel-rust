mod cli;
mod myucel;

use clap::Parser;
use cli::Args;
use colored::Colorize;
use spinners::{Spinner, Spinners};
use crate::myucel::Myucel;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Display a spinner while searching Reddit
    let status_text = format!(
        "Searching Reddit /r/anime for {} discussion threads...",
        args.name.green().bold()
    )
    .blue().to_string();
    let mut spinner = Spinner::new(Spinners::Dots, status_text.into());

    // Search Reddit
    match Myucel::search_reddit(&args.name).await {
        Ok(posts) => {
            spinner.stop();
            println!();
            for post in posts {
                println!("{}: {}", post.title, post.url);
            }
        }
        Err(e) => {
            spinner.stop();
            println!();
            eprintln!("Error searching Reddit: {}", e);
        }
    }
}
