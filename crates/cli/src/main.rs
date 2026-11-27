//! `orchestra`: the headless CLI (WS-02 §3.9). Commands arrive in M4f.
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "orchestra",
    version,
    about = "Orchestra headless simulation CLI"
)]
struct Cli {}

fn main() {
    Cli::parse();
}
