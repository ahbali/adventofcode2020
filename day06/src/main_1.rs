use std::{
    collections::HashSet,
    io::{self, prelude::*},
};
use std::{error::Error, writeln};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input = reader.lines().filter_map(Result::ok);
    let mut result = 0;
    let mut set: HashSet<char> = HashSet::new();

    for line in input {
        if line.len() == 0 {
            result += set.len();
            set.clear();
        }
        set.extend(line.chars());
    }

    writeln!(&mut out, "{:?}", result)?;

    Ok(())
}
