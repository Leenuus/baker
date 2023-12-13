// TODO chezmoi init
// TODO chezmoi add
// TODO chezmoi edit

use std::path::{Path, PathBuf};
use std::process::Stdio;
use clap::{Command, Parser, Subcommand};
use fs_err as fs;
use fs_err::{create_dir_all, File};
use fs_err::PathExt;

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands{
    /// init baker local repo
    Init,
    /// add file to local repo
    Add,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Program {

    #[command(subcommand)]
    sub_commands: Commands
}

fn init(){
    // check whether local repo exist
    let HOME: String = std::env::vars().find(|(name, value)| name == "HOME" ).expect("Can't get HOME path").1;

    let mut repo = PathBuf::from(HOME);
    repo.push(".local/baker");

    match repo.try_exists(){
        Ok(false) => {
            create_dir_all(repo.clone()).expect("Fail to create repo");
            println!("Create REPO in {}", repo.as_os_str().to_str().unwrap());
        },
        Err(e) => {eprintln!("{e:?}")},
        _ => {}
    }

    std::env::set_current_dir(repo).expect("Fail to change work dir");
    let status = std::process::Command::new("git")
        .arg("status")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("git could not be executed");

    if !status.success() {
        let _git = std::process::Command::new("git")
            .stdout(Stdio::inherit())
            .arg("init")
            .status()
            .expect("Not able to create git repo");
    }
}

fn main() {
    let program = Program::parse();

    match &program.sub_commands{
        Commands::Init => init(),
        Commands::Add => {}
    }
}
