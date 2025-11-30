use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "gzy", about = "Make Git commits faster and easier")]
struct CLI {
    #[command(subcommand)]
    commands: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Add { file: String },
    Commit { msg: String },
    Branch { branch_name: String },
    Remote { url: String },
    Push { branch: String },
    Clone { clone_url: String},
    Checkout { branch: String },
    
    Init,
    Status,
    Log,

}

fn main() {
    let cli: CLI = CLI::parse();

    match &cli.commands {
        Cmd::Add { file } => {
            git(&["add", file]);
        }

        Cmd::Commit { msg } => {
            git(&["commit", "-m", msg]);
        }

        Cmd::Branch { branch_name } => {
            git(&["branch", "-M", branch_name]);
        }

        Cmd::Remote { url } => {
            git(&["remote", "add", "origin", url]);
        }

        Cmd::Push { branch } => {
            git(&["push", "-u", "origin", branch]);
        }

        Cmd::Clone { clone_url } => {
            git(&["clone", clone_url]);
        }

        Cmd::Checkout { branch } => {
            git(&["checkout", "-b", branch]);
        }

        Cmd::Init => git(&["init"]),
        Cmd::Status => git(&["status"]),
        Cmd::Log => git(&["log"]),
    }
}

fn git(args: &[&str]) {
    Command::new("git")
        .args(args)
        .status()
        .expect("Failed to execute git command");
}