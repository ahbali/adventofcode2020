use std::writeln;
use std::{collections::HashSet, error::Error};
use std::{
    io::{self, prelude::*},
    unreachable,
};

fn generate_neighbors((x, y, z): &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    let mut ret: Vec<(isize, isize, isize)> = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                if (i, j, k) != (0, 0, 0) {
                    ret.push((x + i, y + j, z + k));
                }
            }
        }
    }
    ret
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut reader = io::BufReader::new(stdin.lock());

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let mut active: HashSet<(isize, isize, isize)> = HashSet::new();
    let mut inactive: HashSet<(isize, isize, isize)> = HashSet::new();

    for (j, line) in buf.lines().enumerate() {
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '#' => active.insert((i as isize, j as isize, 0)),
                '.' => inactive.insert((i as isize, j as isize, 0)),
                _ => unreachable!(),
            };
        }
    }

    for _ in 0..6 {
        let active_copy = active.clone();
        for &i in active.union(&inactive.clone()) {
            for nei in generate_neighbors(&i) {
                if !active.contains(&nei) {
                    inactive.insert(nei);
                }
            }
        }
        // let inactive_copy = inactive.clone();
        let cycle = active
            .union(&inactive)
            .cloned()
            .collect::<HashSet<(isize, isize, isize)>>();

        // writeln!(&mut out, "cycle 1st = {:?}", cycle)?;

        for &cube in &cycle {
            let mut active_count = 0;
            for neighbor in generate_neighbors(&cube) {
                if active_copy.contains(&neighbor) {
                    active_count += 1;
                } else {
                    inactive.insert(neighbor);
                }
            }
            if active_copy.contains(&cube) {
                if !(active_count == 2 || active_count == 3) {
                    active.remove(&cube);
                    inactive.insert(cube);
                }
            } else {
                if active_count == 3 {
                    inactive.remove(&cube);
                    active.insert(cube);
                }
            }
        }
    }

    // During a cycle, all cubes simultaneously change their state according to the following rules:

    // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
    // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.

    // How many cubes are left in the "active" state after the sixth cycle?:
    // dummy result 112

    writeln!(&mut out, "active = {:?}", active.len())?;
    // writeln!(&mut out, "inactive = {:?}", inactive)?;

    Ok(())
}
