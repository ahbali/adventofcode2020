use std::io::{self, prelude::*};
use std::writeln;
use std::{collections::VecDeque, error::Error};

const WINDOW: usize = 25;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let mut input = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.trim().parse::<usize>().ok());

    let mut queue: VecDeque<usize> = input.by_ref().take(WINDOW).collect();

    for number in input {
        let mut temp_buffer: Vec<&usize> = queue.iter().filter(|&x| x < &number).collect();
        temp_buffer.sort();
        let mut found: Option<&usize> = None;
        for &candidate in temp_buffer.iter() {
            let maybe = number - candidate;
            if let Ok(_) = temp_buffer.binary_search(&&maybe) {
                found = Some(candidate);
                break;
            }
        }
        if found == None {
            writeln!(&mut out, "number = {:?}", number)?;
        }
        // writeln!(&mut out, "number = {:?}", number)?;
        // writeln!(&mut out, "queue = {:?}", queue)?;
        // writeln!(&mut out, "vector = {:?}", temp_buffer)?;
        queue.push_back(number);
        queue.pop_front();
        // writeln!(&mut out, "***********")?;
    }

    // let result = 0;

    // writeln!(&mut out, "{:?}", result)?;

    Ok(())
}
