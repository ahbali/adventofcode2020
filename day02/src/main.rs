use std::io::{self, prelude::*};
use std::{error::Error, writeln};

fn main() -> Result<(), Box<dyn Error>> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input = reader.lines().filter_map(Result::ok);

    let mut result: usize = 0;
    for line in input {
        let mut line_iter = line.split(":");
        let mut rule_iter = line_iter.next().unwrap().split_whitespace();
        let password = line_iter.next().unwrap().trim();
        let mut range_iter = rule_iter.next().unwrap().trim().split("-");
        let letter = rule_iter.next().unwrap().trim().parse::<char>()?;
        let pos1 = range_iter.next().unwrap().parse::<usize>()?;
        let pos2 = range_iter.next().unwrap().parse::<usize>()?;
        let a = password.chars().nth(pos1 - 1);
        let b = password.chars().nth(pos2 - 1);
        if (a == Some(letter)) ^ (b == Some(letter)) {
            result += 1;
        }
    }

    writeln!(&mut out, "{:?}", result)?;

    Ok(())
}
