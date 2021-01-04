use std::io::{self, prelude::*};
use std::writeln;
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut reader = io::BufReader::new(stdin.lock());

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    // actual input = Your puzzle input is 1,20,8,12,0,14.
    // let input = [0, 3, 6];
    let input: Vec<usize> = buf
        .trim()
        .split(",")
        .filter_map(|num| num.parse::<usize>().ok())
        .collect();
    let len = input.len();

    let mut game: HashMap<usize, usize> = input
        .iter()
        .enumerate()
        .map(|(idx, &val)| (val, idx))
        .take(len - 1)
        .collect();

    println!("game = {:?}", game);
    let mut prev: (usize, usize) = (input[len - 1], len - 1);
    println!("first prev= {:?}", prev);

    for i in len..30000000 {
        match game.get(&prev.0) {
            Some(&k) => {
                game.insert(prev.0, prev.1);
                prev = (prev.1 - k, i);
            }
            None => {
                game.insert(prev.0, prev.1);
                prev = (0, i);
            }
        }
    }

    writeln!(&mut out, "result = {:?}", prev.0)?;
    Ok(())
}
// run in "debug" mode is too slow, you should run this in "release" mode
// cargo run --release
