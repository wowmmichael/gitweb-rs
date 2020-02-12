#[macro_use]
extern crate lazy_static;

use crate::git_util::derive_repo_url;

#[macro_use]
mod command;
mod git_util;

fn open_chrome(target: &str) {
    run_command!("open", "-a", "google chrome", target);
}

fn read_git_remote() -> Option<String> {
    run_command!("git", "remote").get(0)
        .and_then(|r| run_command!("git", "remote", "get-url", r).pop())
        .map(|r| derive_repo_url(r).expect("invalid remote addr"))
}

fn main() {
    match read_git_remote() {
        Some(x) => {
            println!("remote: {}", x);
            open_chrome(&x)
        }
        None => println!("no remote is found!")
    }
}