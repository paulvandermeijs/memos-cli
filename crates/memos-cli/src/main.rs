mod auth;
mod commands;
mod editor;
mod io;

use clap::{Parser, Subcommand};
use clap_verbosity::Verbosity;

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

    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login to your server
    Login { host: Option<String> },
    /// List memos
    #[command(visible_alias = "ls")]
    List,
}

fn main() {
    let auth = auth::Auth::read();
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let result = match cli.command {
        Some(Commands::Login { host }) => commands::login::login(auth, host),
        Some(Commands::List) => commands::list::list(auth),
        None => commands::create::create(auth, cli.no_edit, cli.workspace, cli.public),
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
