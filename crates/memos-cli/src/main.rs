mod auth;
mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Memos CLI", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Login { host: Option<String> },
    List,
}

fn main() {
    let auth = auth::Auth::read();
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Login { host }) => commands::login::login(auth, host),
        Some(Commands::List) => commands::list::list(&auth),
        None => panic!("Not implemented"),
    };

    let code = match result {
        Ok(_) => 0,
        Err(message) => {
            eprintln!("{}", message);

            1
        }
    };

    std::process::exit(code);
}
