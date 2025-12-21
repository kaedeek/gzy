use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "gzy",
    about = "Make Git commits faster and easier"
)]
struct CLI {
    #[command(subcommand)]
    commands: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Add {
        file: String,
    },

    Commit {
        msg: String,
    },

    Branch {
        branch_name: String,
    },

    Remote {
        url: String,
    },

    Push {
        branch: Option<String>,

        #[arg(short = 'u', long = "set-upstream")]
        upstream: bool,

        #[arg(short = 'f', long = "force")]
        force: bool,
    },

    Clone {
        clone_url: String,
    },

    Checkout {
        branch: String,

        #[arg(short = 'b', long = "create")]
        create: bool,
    },

    Init,
    Status,
    Log,
}

fn main() {
    let cli = CLI::parse();

    match &cli.commands {
        Cmd::Add { file } => {
            git(vec!["add".into(), file.clone()]);
        }

        Cmd::Commit { msg } => {
            git(vec!["commit".into(), "-m".into(), msg.clone()]);
        }

        Cmd::Branch { branch_name } => {
            git(vec![
                "branch".into(),
                "-M".into(),
                branch_name.clone(),
            ]);
        }

        Cmd::Remote { url } => {
            git(vec![
                "remote".into(),
                "add".into(),
                "origin".into(),
                url.clone(),
            ]);
        }

        Cmd::Push {
            branch,
            upstream,
            force,
        } => {
            let mut args = vec!["push".to_string()];

            if *upstream {
                args.push("-u".into());
                args.push("origin".into());
            }

            if let Some(branch) = branch {
                args.push(branch.clone());
            }

            if *force {
                args.push("--force".into());
            }

            git(args);
        }

        Cmd::Clone { clone_url } => {
            git(vec!["clone".into(), clone_url.clone()]);
        }

        Cmd::Checkout { branch, create } => {
            if *create {
                git(vec![
                    "checkout".into(),
                    "-b".into(),
                    branch.clone(),
                ]);
            } else {
                git(vec![
                    "checkout".into(),
                    branch.clone(),
                ]);
            }
        }

        Cmd::Init => {
            git(vec!["init".into()]);
        }

        Cmd::Status => {
            git(vec!["status".into()]);
        }

        Cmd::Log => {
            git(vec!["log".into()]);
        }
    }
}

fn git(args: Vec<String>) {
    Command::new("git")
        .args(args)
        .status()
        .expect("Failed to execute git command");
}