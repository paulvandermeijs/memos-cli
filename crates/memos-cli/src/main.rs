mod auth;
mod commands;

use clap::{Parser, Subcommand};

/// Create and browse your memos
#[derive(Parser, Debug)]
#[command(name = "Memos CLI", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Don't open the editor
    #[arg(short, long)]
    no_edit: bool,

    /// Set the visibility for new memos to "workspace"
    #[arg(short, long)]
    workspace: bool,

    /// Set the visibility for new memos to "public"
    #[arg(short, long)]
    public: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login to your server
    Login { host: Option<String> },
    /// List memos
    List,
}

fn main() {
    let auth = auth::Auth::read();
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Login { host }) => commands::login::login(auth, host),
        Some(Commands::List) => commands::list::list(&auth),
        None => commands::create::create(&auth, cli.no_edit, cli.workspace, cli.public),
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
