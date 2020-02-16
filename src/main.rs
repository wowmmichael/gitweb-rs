#[macro_use]
extern crate lazy_static;

use std::process::exit;

use crate::git_util::derive_repo_url;
use crate::prompt::PromptForChoice;

#[macro_use]
mod command;
mod git_util;
mod prompt;

fn open_chrome(target: &str) {
    println!("Opening {}", target);
    run_command!("open", "-a", "google chrome", target);
}

fn read_git_remote() -> Vec<String> {
    run_command!("git", "remote")
        .iter()
        .map(|r| run_command!("git", "remote", "get-url", r).pop().unwrap())
        .map(|r| derive_repo_url(r))
        .collect()
}

fn main() {
    let remotes = read_git_remote();
    if remotes.is_empty() {
        println!("no remote is found!");
        exit(1);
    }

    let r = (&remotes).into_iter().collect();
    let chosen_remote = *PromptForChoice::new(r).choose();
    open_chrome(&chosen_remote);
}