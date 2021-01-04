use std::error::Error;
use std::io::{self, prelude::*};
use std::writeln;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let estimation = input[0].trim().parse::<usize>().unwrap();
    let buses: Vec<usize> = input[1]
        .trim()
        .split(',')
        .filter_map(|k| k.parse::<usize>().ok())
        .collect();

    // Multiplying the bus ID by the number of minutes you'd need to wait gives 295.
    if let Some((id, result)) = buses
        .iter()
        .map(|k| (k, (k - estimation % k)))
        .min_by_key(|&(_, y)| y)
    {
        writeln!(&mut out, "result = {:?}", result * id)?;
    };
    // writeln!(&mut out, "estimation = {:?}", estimation)?;
    // writeln!(&mut out, "buses = {:?}", buses)?;

    Ok(())
}
