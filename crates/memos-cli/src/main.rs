use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Memos CLI", version, about, long_about = None)]
struct Args {
}

fn main() {
    let _args = Args::parse();
}
