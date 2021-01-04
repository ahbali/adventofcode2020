use std::error::Error;
use std::io::{self, prelude::*};
use std::writeln;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let mut input: Vec<usize> = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.trim().parse::<usize>().ok())
        .collect();

    input.sort();
    let mut previous = 0;
    let mut ones: usize = 0;
    let mut threes: usize = 1;

    for &current in input.iter() {
        let difference = current - previous;
        previous = current;
        match difference {
            1 => {
                ones += 1;
            }
            3 => {
                threes += 1;
            }
            _ => continue,
        }
    }
    let result = ones * threes;
    writeln!(&mut out, "{:?}", result)?;

    Ok(())
}
