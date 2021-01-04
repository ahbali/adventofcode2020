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
    // considering the most recently spoken number:

    // If that was the first time the number has been spoken, the current player says 0.
    // Otherwise, the number had been spoken before; the current player announces
    //  how many turns apart the number is from when it was previously spoken.

    // solution to the example = 436

    // Here are a few more examples:

    // Given the starting numbers 1,3,2, the 2020th number spoken is 1.
    // Given the starting numbers 2,1,3, the 2020th number spoken is 10.
    // Given the starting numbers 1,2,3, the 2020th number spoken is 27.
    // Given the starting numbers 2,3,1, the 2020th number spoken is 78.
    // Given the starting numbers 3,2,1, the 2020th number spoken is 438.
    // Given the starting numbers 3,1,2, the 2020th number spoken is 1836.

    // actual input = Your puzzle input is 1,20,8,12,0,14.
    // let input = [0usize, 3usize, 6usize];
    let input: Vec<usize> = buf
        .trim()
        .split(",")
        .filter_map(|num| num.parse::<usize>().ok())
        .collect();
    // let input = [0, 3, 6];
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

    for i in len..2020 {
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
        // println!("game = {:?}", game);
        // println!("prev = {:?}", prev);
    }

    // println!("{:?}", prev);
    writeln!(&mut out, "result = {:?}", prev.0)?;
    Ok(())
}
