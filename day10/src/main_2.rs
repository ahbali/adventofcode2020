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

    let mut input: Vec<usize> = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.trim().parse::<usize>().ok())
        .collect();

    input.push(0);
    input.sort();
    let last: usize = input.iter().max().clone().unwrap() + 3;
    input.push(last);

    writeln!(&mut out, "sorted        ={:?}", input)?;
    let mut previous = &0;
    let mut ones: HashSet<&usize> = HashSet::new();
    let mut threes: HashSet<&usize> = HashSet::new();
    let mut must_be_present: HashSet<&usize> = HashSet::new();
    must_be_present.insert(&0);

    for current in input.iter().skip(1) {
        let difference = current - previous;
        match difference {
            1 => {
                ones.insert(current);
            }
            3 => {
                threes.insert(current);
                must_be_present.insert(current);
                must_be_present.insert(previous);
            }
            _ => unreachable!(),
        }
        previous = current;
    }

    let result = ones.len() * (threes.len() + 1);

    let free_set: HashSet<&usize> = ones.difference(&must_be_present).cloned().collect();

    let mut free: Vec<&usize> = free_set.iter().cloned().collect();
    free.sort();

    let mut ones_vec: Vec<&usize> = ones.iter().cloned().collect();
    let mut threes_vec: Vec<&usize> = threes.iter().cloned().collect();
    let mut must_be_present_vec: Vec<&usize> = must_be_present.iter().cloned().collect();
    ones_vec.sort();
    threes_vec.sort();
    must_be_present_vec.sort();

    let mut ones_places: Vec<(&usize, &usize)> = vec![];

    for (i, j) in must_be_present_vec.windows(2).map(|win| (win[0], win[1])) {
        if j - i > 3 {
            ones_places.push((i, j));
        }
    }

    writeln!(&mut out, "ones          ={:?}", ones_vec)?;
    writeln!(&mut out, "threes        ={:?}", threes_vec)?;
    writeln!(&mut out, "")?;
    writeln!(&mut out, "must be       ={:?}", must_be_present_vec)?;
    writeln!(&mut out, "free          ={:?}", free)?;
    writeln!(&mut out, "ones places   ={:?}", ones_places)?;
    writeln!(&mut out, "{:?}", result)?;
    let mut res = 7usize.pow(ones_places.len() as u32);

    let mut result_set: HashSet<&usize> = HashSet::new();
    let mut result_map: HashMap<(usize, usize), usize> = HashMap::new();
    // let (mut i, mut j): (usize, usize) = (0, 0);
    for (&i, &j) in ones_places {
        let v = (i..=j).filter_map(|element| free_set.get(&element));
        result_set.extend(v);
        // result_map.insert((i, j), v);
    }

    let d: HashSet<&usize> = free_set.difference(&result_set).cloned().collect();
    res = res * 2usize.pow(d.len() as u32);
    writeln!(&mut out, "result set   ={:?}", result_set)?;
    writeln!(&mut out, "difference   ={:?}", d)?;
    writeln!(&mut out, "result_map   ={:?}", result_map)?;
    writeln!(
        &mut out,
        "****** this is what we want ****** ={:?}  *********",
        res
    )?;

    Ok(())
}
