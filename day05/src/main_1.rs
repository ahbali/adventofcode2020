use std::io::{self, prelude::*};
use std::{error::Error, writeln};

fn main() -> Result<(), Box<dyn Error>> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    // this give the same result as bellow

    // let result = reader
    //     .lines()
    //     .filter_map(Result::ok)
    //     .map(|line| {
    //         line.replace("F", "0")
    //             .replace("B", "1")
    //             .replace("L", "0")
    //             .replace("R", "1")
    //     })
    //     .map(|line| usize::from_str_radix(&line, 2))
    //     .filter_map(Result::ok)
    //     .max()
    //     .unwrap();

    let input = reader.lines().filter_map(Result::ok).map(|line| {
        line.replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1")
    });
    let mut result = 0;
    for line in input {
        let (row_str, col_str) = line.split_at(7);
        let row = usize::from_str_radix(row_str, 2)?;
        let col = usize::from_str_radix(col_str, 2)?;
        let id = row * 8 + col;
        if result < id {
            result = id;
        }
    }

    writeln!(&mut out, "{}", result)?;

    Ok(())
}
