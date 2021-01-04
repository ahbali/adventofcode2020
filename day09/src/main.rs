use std::writeln;
use std::{collections::VecDeque, error::Error};
use std::{
    io::{self, prelude::*},
    unreachable,
};

const WINDOW: usize = 25;

fn find_invalid(input: &Vec<usize>) -> Option<(usize, &usize)> {
    let mut queue: VecDeque<&usize> = input.iter().take(WINDOW).collect();

    for (idx, number) in input.iter().enumerate().skip(WINDOW) {
        let mut temp_buffer: Vec<&&usize> = queue.iter().filter(|&x| x < &number).collect();
        temp_buffer.sort();
        let mut found: Option<&usize> = None;
        for &&candidate in temp_buffer.iter() {
            let maybe = number - candidate;
            if let Ok(_) = temp_buffer.binary_search(&&&maybe) {
                found = Some(candidate);
                break;
            }
        }
        if found == None {
            return Some((idx, number));
        }
        queue.push_back(number);
        queue.pop_front();
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input: Vec<usize> = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.trim().parse::<usize>().ok())
        .collect();

    let (idx, invalid_number) = find_invalid(&input).unwrap();

    let mut queue: VecDeque<&usize> = VecDeque::new();
    let mut len = 0;
    let mut result = 0;
    writeln!(&mut out, "invalid_number = {:?}", invalid_number)?;

    let mut input_iter = input.iter();
    let mut counter = 0;

    // for _ in 0..=idx {
    while input_iter.len() > 0 {
        counter += 1;
        let sum: usize = queue.iter().cloned().sum();

        match sum {
            k if &k == invalid_number => {
                writeln!(&mut out, "==")?;
                if len <= queue.len() {
                    len = queue.len();
                    let min = queue.iter().cloned().min().unwrap();
                    let max = queue.iter().cloned().max().unwrap();
                    result = min + max;
                };
                queue.push_back(input_iter.next().unwrap());
            }
            w if &w < invalid_number => {
                // writeln!(&mut out, "<")?;
                queue.push_back(input_iter.next().unwrap());
            }
            z if &z > invalid_number => {
                // writeln!(&mut out, ">")?;
                queue.pop_front();
            }
            _ => unreachable!(),
        }
    }

    writeln!(
        &mut out,
        "index from function = {:?} steps = {:?} len = {:?} , result = {:?}",
        idx, counter, len, result
    )?;

    Ok(())
}
