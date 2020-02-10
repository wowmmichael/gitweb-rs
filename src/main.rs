#[macro_use]
mod command;

fn open_chrome(target: &str) {
    run_command!("open", "-a", "google chrome", target);
}

fn main() {
    let s = run_command!("ls", "-alth");

    for (i, l) in s.iter().enumerate() {
        println!("finished with line {}: {}", i, *l);
    }
}
