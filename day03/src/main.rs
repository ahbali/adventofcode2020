use std::io::{self, prelude::*};
use std::{error::Error, writeln};

fn main() -> Result<(), Box<dyn Error>> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input = reader.lines().filter_map(Result::ok);

    let mut positions = [0; 5];
    let mut tree_counts: [usize; 5] = [0; 5];
    // let length = input.next().unwrap().len();
    let cases = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (line_number, line) in input.enumerate() {
        for (idx, &(right, down)) in cases.iter().enumerate() {
            if line_number % down == 0 && line_number > 0 {
                positions[idx] = (positions[idx] + right) % line.len();
                if line.chars().nth(positions[idx]) == Some('#') {
                    tree_counts[idx] += 1;
                }
            }
        }
    }

    writeln!(&mut out, "{:?}", tree_counts.iter().product::<usize>())?;

    Ok(())
}
