use std::io::{self, prelude::*};
use std::writeln;
use std::{error::Error, ops::RangeInclusive};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut reader = io::BufReader::new(stdin.lock());

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let mut input = buf.split("\n\n");

    let ranges_str = input.next().unwrap();
    // class: 1-3 or 5-7
    // row: 6-11 or 33-44
    // seat: 13-40 or 45-50
    let _my_ticket_str = input.next().unwrap();
    // your ticket:
    // 7,1,14
    let nearby_tickets_str = input.next().unwrap();
    // nearby tickets:
    // 7,3,47
    // 40,4,50
    // 55,2,20
    // 38,6,12

    let ranges_usize = ranges_str
        .lines()
        .flat_map(|line| line.split(": ").nth(1))
        .flat_map(|line| line.split(" or "))
        .flat_map(|rg| rg.split("-"))
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let ranges: Vec<RangeInclusive<usize>> = ranges_usize
        .chunks_exact(2)
        .map(|sl| (sl[0]..=sl[1]))
        .collect();

    let tickets_usize = nearby_tickets_str
        .lines()
        .map(|line| line.split(",").filter_map(|s| s.parse::<usize>().ok()));

    let mut error_rate: usize = 0;
    for ticket in tickets_usize {
        for number in ticket {
            let mut exist = false;
            for range in ranges.iter() {
                if range.contains(&number) {
                    exist = true;
                    break;
                }
            }
            if !exist {
                error_rate += number;
            }
        }
    }
    writeln!(&mut out, "error rate = {}", error_rate)?;
    Ok(())
}
