use std::{fs::File, io::Read, path::PathBuf};

use anyhow::Result;
use argh::FromArgs;
use itertools::Itertools;

/// Inputs results
#[derive(FromArgs)]
struct Input {
    /// path to input file of passwords
    #[argh(positional)]
    path: PathBuf,
}

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

impl Policy {
    fn is_password_valid(&self, password: &str) -> bool {
        password
            .chars()
            .enumerate()
            .filter(|&(i, _)| i == self.min || i == self.max)
            .filter(|&(_, c)| c == self.letter)
            .count()
            == 1
    }
}

fn parse_line(input: &str) -> Result<(Policy, &str)> {
    peg::parser! {
        grammar password_parser() for str {
            rule number() -> usize = n:$(['0'..='9']+) { n.parse().unwrap() }
            rule position() -> usize = n:number() { n - 1 }
            rule char() -> char = letter:$(['a'..='z']) { letter.chars().next().unwrap() }
            rule password() -> &'input str = letters:$([_]*) { letters }

            pub(crate) rule line() -> (Policy, &'input str)
                = min:position() "-" max:position() " " letter:char() ": " password:password() {
                    (Policy {min, max, letter}, password)
                }
        }
    }

    Ok(password_parser::line(input)?)
}

fn main() -> Result<()> {
    let input: Input = argh::from_env();
    let mut contents = String::new();
    File::open(input.path)?.read_to_string(&mut contents)?;
    let valid_password_count = contents
        .lines()
        .map(|line| parse_line(&line))
        .flatten()
        .filter(|(policy, password)| policy.is_password_valid(password))
        .count();
    dbg!(valid_password_count);
    Ok(())
}
