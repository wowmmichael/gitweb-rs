use std::ffi::OsStr;
use std::iter::once;
use std::process::Command;


fn run_command_with_args<S>(cmd: S, args: &Vec<S>) -> Vec<String>
where S: AsRef<OsStr>
{
    let cmd_repr = once(&cmd).chain(args.iter())
        .map(|a| a.as_ref().to_os_string().into_string().unwrap())
        .collect::<Vec<String>>().join(" ");

    let output = Command::new(&cmd).args(args).output()
        .expect(&format!("cannot run command `{}`", cmd_repr));

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).ok()
            .unwrap_or(String::default());

        panic!("cannot run command `{}` with error:\n{}", cmd_repr, stderr);
    }

    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout.lines().map(|s| s.to_owned()).collect::<Vec<_>>()
}

fn run_command_without_args<S>(cmd: S) -> Vec<String>
where S: AsRef<OsStr>
{
    let args: Vec<S> = Vec::default();
    run_command_with_args(cmd, &args)
}

macro_rules! run_command {
    ($cmd:expr) => {
        run_command_without_args($cmd)
    };
    ($cmd:expr, $($arg:expr),+) => {
        run_command_with_args($cmd, &vec![$($arg),*])
    }
}

fn open_chrome(target: &str) {
    run_command!("open", "-a", "google chrome", target);
}


fn main() {

    let s = run_command!("git", "remote");

    for (i, l) in s.iter().enumerate() {
        println!("finished with line {}: {}", i, *l);
    }
}
