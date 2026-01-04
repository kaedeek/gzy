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
    Add { file: String },
    Commit { msg: String },
    Branch { branch_name: String },
    Remote { url: String },

    Push {
        branch: Option<String>,

        #[arg(short = 'u', long = "set-upstream")]
        upstream: bool,

        #[arg(short = 'f', long = "force")]
        force: bool,
    },

    Clone { clone_url: String },

    Checkout {
        branch: String,

        #[arg(short = 'b', long = "create")]
        create: bool,
    },

    Conflict {
        strategy: String,
    },

    Sync {
        base: String,
        branch: Option<String>,
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
            git(vec!["branch".into(), "-M".into(), branch_name.clone()]);
        }

        Cmd::Remote { url } => {
            git(vec![
                "remote".into(),
                "add".into(),
                "origin".into(),
                url.clone(),
            ]);
        }

        Cmd::Push { branch, upstream, force } => {
            let mut args: Vec<String> = vec!["push".into()];

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
                git(vec!["checkout".into(), "-b".into(), branch.clone()]);
            } else {
                git(vec!["checkout".into(), branch.clone()]);
            }
        }

        Cmd::Conflict { strategy } => match strategy.as_str() {
            "ours" => {
                git(vec!["checkout".into(), "--ours".into(), ".".into()]);
                git(vec!["add".into(), ".".into()]);
                git(vec![
                    "commit".into(),
                    "-m".into(),
                    "Resolve conflicts (ours)".into(),
                ]);
            }
            "theirs" => {
                git(vec!["checkout".into(), "--theirs".into(), ".".into()]);
                git(vec!["add".into(), ".".into()]);
                git(vec![
                    "commit".into(),
                    "-m".into(),
                    "Resolve conflicts (theirs)".into(),
                ]);
            }
            "abort" => {
                git(vec!["merge".into(), "--abort".into()]);
            }
            _ => {
                eprintln!("strategy must be: ours | theirs | abort");
            }
        },

        Cmd::Sync { base, branch } => {
            git(vec!["fetch".into(), "upstream".into()]);
            git(vec!["checkout".into(), base.clone()]);
            git(vec![
                "rebase".into(),
                format!("upstream/{}", base),
            ]);

            let target = match branch {
                Some(b) => b.clone(),
                None => {
                    let output = Command::new("git")
                        .args(["branch", "--show-current"])
                        .output()
                        .expect("failed to get current branch");
                    String::from_utf8_lossy(&output.stdout).trim().to_string()
                }
            };

            git(vec!["checkout".into(), target.clone()]);
            git(vec!["rebase".into(), base.clone()]);
            git(vec![
                "push".into(),
                "origin".into(),
                target,
                "--force-with-lease".into(),
            ]);
        }

        Cmd::Init => git(vec!["init".into()]),
        Cmd::Status => git(vec!["status".into()]),
        Cmd::Log => git(vec!["log".into()]),
    }
}

fn git(args: Vec<String>) {
    Command::new("git")
        .args(args)
        .status()
        .expect("Failed to execute git command");
}
