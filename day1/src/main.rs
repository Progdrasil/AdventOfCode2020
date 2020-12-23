use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use argh::FromArgs;
use itertools::Itertools;

#[derive(FromArgs)]
/// Find the numbers which their sum returns 2020
struct Input {
    /// path to input file, must be formatted as a single positive integer per line
    #[argh(positional)]
    path: PathBuf,
    /// combination size
    #[argh(option, default = "2")]
    combos: usize,
}

fn main() -> Result<()> {
    let input: Input = argh::from_env();
    let numbers = BufReader::new(File::open(input.path)?)
        .lines()
        .flatten()
        .map(|l| l.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;

    let winner = numbers
        .into_iter()
        .combinations(input.combos)
        .find(|combo| combo.iter().sum::<usize>() == 2020)
        .ok_or_else(|| anyhow!("No combinations sum up to 2020"))?;

    dbg!(&winner);
    dbg!(winner.into_iter().product::<usize>());

    Ok(())
}
