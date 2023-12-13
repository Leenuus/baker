// TODO chezmoi init
// TODO chezmoi add
// TODO chezmoi edit

use std::path::{Path, PathBuf};
use std::io::Write;
use std::process::Stdio;
use clap::{Parser, Subcommand};
use fs_err::create_dir_all;
use fs_err::File;
extern crate fs_extra;
use fs_extra::dir;
use fs_extra::TransitProcess;
use fs_extra::file;
use fs_extra::error::*;
use fsio::file::{write_text_file, append_text_file};
use normpath::PathExt as NormalPathExt;
use relative_path::PathExt;
use relative_path::RelativePath;


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

const BAKER_ROOT: &'static str = "./.local/baker";
const REPO_PATH: &'static str = "./.local/baker/repo";
const BAKER_INDEX: &'static str = "./.local/baker/index";

struct Index{
    file_name: String,
    file_path: String
}

fn get_home_path() -> PathBuf{
    PathBuf::from(std::env::var("HOME").expect("Fail to get HOME path"))
}


fn get_baker_root() -> PathBuf{
    let home = get_home_path();
    let baker = RelativePath::new(BAKER_ROOT);
    let baker_root = baker.to_path(home);
    baker_root
}

fn get_repo_path() -> PathBuf{
    let home = get_home_path();
    let repo = RelativePath::new(REPO_PATH);
    let repo = repo.to_path(home);
    repo
}

fn get_baker_index() -> PathBuf{
    let home = get_home_path();
    let index = RelativePath::new(BAKER_INDEX);
    let index = index.to_path(home);
    index
}

fn check_baker_health() -> bool {
    // TODO I need a more comprehensive health check function
    let root = get_baker_root();
    let index= get_baker_index();
    let repo = get_repo_path();
    root.is_dir() && index.is_file() && repo.is_dir()
}

fn init(){
    // check whether local repo exist
    let repo = get_repo_path();
    let baker_root = get_baker_root();

    if !repo.is_dir(){
        create_dir_all(&repo).expect("Fail to create repo");
        println!("Create REPO in {}", repo.as_os_str().to_str().unwrap());
    }

    std::env::set_current_dir(baker_root).expect("Fail to change working dir");
    let status = std::process::Command::new("git")
        .arg("status")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("baker depends on git, please install git first");

    // TODO I am not sure without git installed, what status will be in fact
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
    // FIXME when filename starts with "dot", two filenames get conflicts
    if name.as_ref().starts_with("."){
        name.as_ref().replacen(".", "dot_", 1)
    }else{
        name.as_ref().to_string()
    }
}

fn get_copy_path(abpath: &Path) -> PathBuf{
    let repo = get_repo_path();
    let home = get_home_path();
    let file_relative_path_to_home = abpath.relative_to(home).expect("Bad file path");
    // if regular file
    let copy_path = file_relative_path_to_home.to_path(&repo);
    copy_path
}

fn add(name: String){
    // check whether repo exists
    let baker = get_baker_root();
    if !baker.exists(){
        println!("REPO not exists! Please call baker init first");
        return;
    }
    let index = get_baker_index();
    let home = get_home_path();
    let repo = get_repo_path();


    let mut cwd = PathBuf::from(std::env::current_dir().unwrap());
    cwd.push(&name);
    let cur_file = cwd.as_path();
    let cur_file = cur_file.normalize().unwrap();
    let cur_file = cur_file.as_path();
    let file_relative_path_to_home = cur_file.relative_to(home).expect("baker only helps with your home directory file!").to_path(".");

    match cur_file.try_exists(){
        Ok(true) => {
            if cur_file.is_file() {
                // if regular file
                let mut copy_path = get_copy_path(cur_file);
                
                // create dir to hold the file
                let copy_root = copy_path.parent().unwrap();
                fs_err::create_dir_all(copy_root).expect("Fail to create copy");

                // santisize filename
                let file_name = rename_dot_file(copy_path.file_name().unwrap().to_str().unwrap());
                copy_path.pop();
                copy_path = copy_path.join(file_name);

                // copy file
                let mut copy_options = file::CopyOptions::new();
                copy_options.skip_exist = false;
                copy_options.overwrite = true;
                file::copy_with_progress(cur_file, &copy_path, &copy_options, progress_handle).unwrap();

                // add to index
                // FIXME remove repeat index
                // better index with a small database

                let copy_path = copy_path.relative_to(repo).unwrap();
                let copy_path = copy_path.to_path(".");
                let index_entry = format!("\n{}|{}", file_relative_path_to_home.to_str().unwrap(), &copy_path.to_str().unwrap());
                append_text_file(&index, &index_entry).unwrap();

            }else if cur_file.is_dir(){
                // if dir
                // TODO
                let copy_options = dir::CopyOptions::new();
                // TODO name, deal with conflicts
            }else if cur_file.is_symlink(){
            }
        }
        Ok(false) => {
            println!("File does not exist");
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
