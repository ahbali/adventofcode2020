use std::io::{self, prelude::*};
use std::{error::Error, writeln};

fn main() -> Result<(), Box<dyn Error>> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input = reader.lines().filter_map(Result::ok);

    let mut position = 0;
    let mut tree_count: usize = 0;
    for line in input.skip(1) {
        position = (position + 3) % line.len();
        if line.chars().nth(position) == Some('#') {
            tree_count += 1;
        }
    }

    writeln!(&mut out, "{:?}", tree_count)?;

    Ok(())
}
