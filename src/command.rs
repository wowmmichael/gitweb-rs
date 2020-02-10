use std::ffi::OsStr;
use std::iter::once;
use std::process::Command;

pub fn run_command_with_args<S>(cmd: S, args: &Vec<S>) -> Vec<String>
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

pub fn run_command_without_args<S>(cmd: S) -> Vec<String>
    where S: AsRef<OsStr>
{
    let args: Vec<S> = Vec::default();
    run_command_with_args(cmd, &args)
}

#[macro_export]
macro_rules! run_command {
    ($cmd:expr) => {
        $crate::command::run_command_without_args($cmd)
    };
    ($cmd:expr, $($arg:expr),+) => {
        $crate::command::run_command_with_args($cmd, &vec![$($arg),*])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_clean() {
        let s = run_command!("ls");
        println!("{:?}", s);
    }
}
