use std::io::{self, prelude::*};
use std::{error::Error, writeln};

fn main() -> Result<(), Box<dyn Error>> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());
    let mut buffer = String::new();

    let mut prev: Vec<usize> = Vec::new();

    let input = reader
        .lines()
        .filter_map(Result::ok)
        .map(|num| num.parse::<usize>())
        .filter_map(Result::ok);
    let mut result: usize = 0;
    for value in input {
        prev.push(value);
        let remaining = 2020 - value;
        for &compare in prev.iter() {
            if compare == remaining {
                result = value * compare;
            }
        }
    }

    writeln!(&mut out, "{:?}", result)?;

    Ok(())
}
