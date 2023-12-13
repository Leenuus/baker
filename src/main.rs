// TODO chezmoi init
// TODO chezmoi add
// TODO chezmoi edit

use std::path::{Path, PathBuf};
use std::io::Write;
use std::process::Stdio;
use clap::{Parser, Subcommand};
use fs_err::create_dir_all;
use fs_err::File;
use fs_err::PathExt;
extern crate fs_extra;
use fs_extra::dir;
use fs_extra::TransitProcess;
use fs_extra::file;
use fs_extra::error::*;
use fsio::file::{write_text_file, append_text_file};

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands{
    /// init baker local repo
    Init,
    /// add file to local repo
    Add {name: String},
    /// edit stored files
    Edit,
    /// get help of other subcommands
    Cd,
    /// load repo to current system
    Load
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Program {
    #[command(subcommand)]
    sub_commands: Commands
}

const BAKER_ROOT: &'static str = ".local/baker";
const REPO_PATH: &'static str = ".local/baker/repo";
const BAKER_INDEX: &'static str = ".local/baker/index";

struct Index{
    file_name: String,
    file_path: String
}

fn get_baker_root() -> PathBuf{
    let home: String = std::env::vars().find(|(name, _)| name == "HOME" ).expect("Can't get home path").1;
    let mut repo = PathBuf::from(home);
    repo.push(BAKER_ROOT);
    repo
}

fn get_repo_path() -> PathBuf{
    let home: String = std::env::vars().find(|(name, _)| name == "HOME" ).expect("Can't get home path").1;
    let mut repo = PathBuf::from(home);
    repo.push(REPO_PATH);
    repo
}

fn get_baker_index() -> PathBuf{
    let home: String = std::env::vars().find(|(name, _)| name == "HOME" ).expect("Can't get home path").1;
    let mut repo = PathBuf::from(home);
    repo.push(BAKER_INDEX);
    repo
}

fn init(){
    // check whether local repo exist
    let repo = get_baker_root();

    if !repo.is_dir(){
        create_dir_all(repo.clone()).expect("Fail to create repo");
        println!("Create REPO in {}", repo.as_os_str().to_str().unwrap());
    }

    std::env::set_current_dir(repo).expect("Fail to change working dir");
    let status = std::process::Command::new("git")
        .arg("status")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("baker depends on git, please install git first");

    if !status.success() {
        let _git = std::process::Command::new("git")
            .stdout(Stdio::inherit())
            .arg("init")
            .status()
            .expect("Not able to create git repo");
    }
}

fn progress_handle(process_info: file::TransitProcess){
    println!("{}/{}", process_info.copied_bytes, process_info.total_bytes);
}

fn rename_dot_file(name: impl AsRef<str>) -> String{
    if name.as_ref().starts_with("."){
        name.as_ref().replacen(".", "dot_", 1)
    }else{
        name.as_ref().to_string()
    }
}

fn pathbuf_to_string(path: &PathBuf) -> &str{
    path.to_str().unwrap()
}

fn add(name: String){
    // check whether repo exists
    let baker = get_baker_root();
    if !baker.exists(){
        println!("REPO not exists! Please call baker init first");
        return;
    }
    let repo = get_repo_path();
    let index = get_baker_index();
    // check whether name is valid file
    // yes: copy file to local repo TODO merge it with local repo edition
    // no: create a file in repo, add reference in database
    let file = Path::new(&name);
    let file_name = file.file_name().unwrap().to_str().unwrap();
    match file.try_exists(){
        Ok(true) => {
            if file.is_file() {
                // if regular file
                let copy_options = file::CopyOptions::new();
                // TODO name, deal with conflicts
                file::copy_with_progress(file, repo.join(file_name), &copy_options, progress_handle).unwrap();
            }else if file.is_dir(){
                // if dir
                // TODO
                let copy_options = dir::CopyOptions::new();
                // TODO name, deal with conflicts
                dir::copy(file, repo.join(file_name), &copy_options).unwrap();
            }else if file.is_symlink(){

            }
        }
        Ok(false) => {
            // create file in repo
            // TODO name, deal with conflicts
            // TODO need real path relative to HOME here
            let name_in_repo = repo.join(file_name);
            let _ = File::create(&name_in_repo).expect("fail to create file");
            // add to index
            append_text_file(&index, &format!("{}|{}", name_in_repo.to_str().unwrap(), file.to_str().unwrap())).unwrap();
            
        }
        _ => {panic!("")}
    }
}


fn main() {
    let program = Program::parse();

    match program.sub_commands{
        Commands::Init => init(),
        Commands::Add{name} => add(name),
        _ => {}
    }
}
