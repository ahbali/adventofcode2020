use std::io::{self, prelude::*};
use std::{error::Error, writeln};

fn main() -> Result<(), Box<dyn Error>> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let mut input = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1")
        })
        .map(|line| usize::from_str_radix(&line, 2))
        .filter_map(Result::ok)
        .collect::<Vec<usize>>();

    input.sort();

    for (index, element) in input.iter().enumerate() {
        if element + 1 != input[index + 1] {
            writeln!(&mut out, "{:?}", element + 1)?;
            break;
        }
    }

    Ok(())
}
