use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::io::{stdin, stdout, Write};

pub struct PromptForChoice<S: AsRef<str>>(Vec<S>);

#[derive(Debug)]
pub struct PromptInputError(String);

impl Error for PromptInputError {}

impl Display for PromptInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid input: {}", &self.0)
    }
}

pub fn prompt_value<S, T, V>(msg: S, parse: T) -> V
    where S: AsRef<str> + Display,
          T: Fn(String) -> Result<V, PromptInputError>
{
    loop {
        print!("{}: ", msg);
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        match parse(line.trim_end().to_string()) {
            Ok(received) => return received,
            Err(e) => println!("{}", e)
        }
    }
}

impl<S: AsRef<str>> PromptForChoice<S> {
    pub fn new(choices: Vec<S>) -> Self {
        Self(choices)
    }

    pub fn choose(&self) -> &S {
        if self.0.is_empty() {
            panic!("Nothing to choose!");
        }

        if self.0.len() == 1 {
            return &self.0[0];
        }

        println!("Please choose from the following items:");

        for (i, v) in self.0.iter().enumerate() {
            println!("\t[{}]: {}", i, v.as_ref());
        }

        let chosen =
            prompt_value(format!("Type choice (0 - {})", self.0.len() - 1), |s| {
                s.parse::<usize>()
                    .map_err(|_| PromptInputError(String::from(&s)))
                    .and_then(|v| {
                        if v < self.0.len() {
                            Ok(v)
                        } else {
                            Err(PromptInputError(String::from(&s)))
                        }
                    })
            });

        &self.0[chosen]
    }
}