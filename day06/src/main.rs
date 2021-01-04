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
    let mut buffer: Vec<String> = Vec::new();

    for line in input {
        if line.len() == 0 {
            let mut prev_set: HashSet<char> = buffer[0].chars().collect();
            for choices in buffer.iter() {
                let next_set: HashSet<char> = choices.chars().collect();
                prev_set = prev_set.intersection(&next_set).cloned().collect();
            }
            result += prev_set.len();
            buffer.clear();
        } else {
            buffer.push(line);
        }
    }

    writeln!(&mut out, "{:?}", result)?;

    Ok(())
}
