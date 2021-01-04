use std::writeln;
use std::{
    collections::HashMap,
    io::{self, prelude::*},
};
use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let mut input: HashSet<usize> = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.trim().parse::<usize>().ok())
        .collect();

    input.insert(0);
    let last = input.iter().cloned().max().unwrap();
    input.insert(last);
    let mut result = 1;
    let mut my_map: HashMap<(usize, usize), Vec<&usize>> = HashMap::new();

    for i in (0..last).step_by(4) {
        let he: Vec<&usize> = (i..i + 4).filter_map(|f| input.get(&f)).collect();
        result = result * (2usize.pow(he.len() as u32) - 1);
        let t = (i, (i + 4));
        my_map.insert(t, he);
    }
    let map_iter = (0..last)
        .map(|k| (k, k + 4))
        .map(|i| my_map.get_key_value(&i));

    writeln!(&mut out, "map ={:?}", map_iter)?;
    writeln!(&mut out, "map ={:?}", my_map)?;
    writeln!(&mut out, "result ={:?}", result)?;

    Ok(())
}
