use std::error::Error;
use std::io::{self, prelude::*};
use std::writeln;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    // let input = String::from("1789,37,47,1889");

    let buses: Vec<(usize, usize)> = input[1]
        .trim()
        .split(',')
        .enumerate()
        .filter(|(_, k)| k.parse::<usize>().is_ok())
        .map(|(id, value)| (id, value.parse::<usize>().unwrap()))
        .collect();

    writeln!(&mut out, "buses = {:?}", buses)?;
    // println!("buses = {:?}", buses);

    let mut estimation = 1usize;
    let mut step = 1;

    for &(remainder, id) in buses.iter() {
        while (estimation + remainder) % id != 0 {
            estimation += step;
        }
        step = step * id;
    }

    writeln!(&mut out, "result = {:?}", estimation)?;

    Ok(())
}
