use std::io::{self, prelude::*};
use std::{error::Error, writeln};

fn main() -> Result<(), Box<dyn Error>> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let mut prev: Vec<usize> = Vec::new();
    let mut other: Vec<usize> = Vec::new();

    let input = reader
        .lines()
        .filter_map(Result::ok)
        .map(|num| num.parse::<usize>())
        .filter_map(Result::ok);
    let mut result: usize = 0;
    for value in input {
        prev.push(value);
        other.push(value);
        for &first_compare in prev.iter() {
            let remaining: i64 = 2020 - (value + first_compare) as i64;
            if remaining > 0 {
                for &second_compare in other.iter() {
                    if second_compare == remaining as usize {
                        result = value * first_compare * second_compare;
                    }
                }
            }
        }
    }

    writeln!(&mut out, "{:?}", result)?;

    Ok(())
}
