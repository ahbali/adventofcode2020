use std::io::{self, prelude::*};
use std::{
    collections::{HashMap, HashSet},
    usize, writeln,
};
use std::{error::Error, ops::RangeInclusive};

fn ticket_to_fields<'a>(
    ticket: &Vec<usize>,
    fields: &'a HashMap<&str, Vec<RangeInclusive<usize>>>,
) -> Option<HashMap<usize, HashSet<&'a str>>> {
    let mut result: HashMap<usize, HashSet<&str>> = HashMap::new();
    let mut found_indexes: HashSet<usize> = HashSet::new();
    for (&field, ranges) in fields {
        for (idx, number) in ticket.iter().enumerate() {
            for range in ranges {
                if range.contains(&number) {
                    result.entry(idx).or_default().insert(field);
                    found_indexes.insert(idx);
                    break;
                }
            }
        }
    }
    if (0..ticket.len())
        .map(|idx| found_indexes.contains(&idx))
        .find(|&x| x == false)
        .is_none()
    {
        return Some(result);
    };
    None
}
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
    let my_ticket: Vec<usize> = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|line| line.split(",").filter_map(|ch| ch.parse::<usize>().ok()))
        .collect();
    // your ticket:
    // 7,1,14
    let nearby_tickets_str = input.next().unwrap();
    // nearby tickets:
    // 7,3,47

    let mut fields: HashMap<&str, Vec<RangeInclusive<usize>>> = HashMap::new();

    for line in ranges_str.lines() {
        let mut line_iter = line.split(": ");
        let field = line_iter.next().unwrap();
        let ranges: Vec<RangeInclusive<usize>> = line_iter
            .next()
            .unwrap()
            .split(" or ")
            .map(|el| {
                el.split("-")
                    .filter_map(|e| e.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .map(|el| (el[0]..=el[1]))
            .collect();

        fields.insert(field, ranges);
    }

    let tickets_iter = nearby_tickets_str
        .lines()
        .skip(1)
        .map(|line| {
            line.split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .filter_map(|ticket| ticket_to_fields(&ticket, &fields));

    let mut final_result: HashMap<usize, HashSet<&str>> = HashMap::new();

    for ticket in tickets_iter {
        for (idx, set) in ticket {
            let tmp = final_result.entry(idx).or_insert(set.clone());
            *tmp = tmp.intersection(&set).cloned().collect::<HashSet<&str>>();
        }
    }

    let mut result_vec: Vec<_> = final_result
        .iter()
        .map(|(idx, set)| (idx, set.clone()))
        .collect();
    result_vec.sort_unstable_by_key(|(_, set)| set.len());

    for idx in 0..result_vec.len() - 1 {
        let element = result_vec[idx].1.iter().cloned().next().unwrap();
        for next in idx + 1..result_vec.len() {
            result_vec[next].1.remove(element);
        }
    }

    let results_map: HashMap<&str, usize> = result_vec
        .iter()
        .map(|(&idx, set)| (set.iter().cloned().next().unwrap(), idx))
        .collect();

    let mut res = 1usize;
    for &key in results_map.keys() {
        if key.starts_with("departure") {
            res *= my_ticket[results_map[key]];
        }
    }
    writeln!(&mut out, "result_vec = {:?}", results_map)?;
    writeln!(&mut out, "final_result = {:?}", res)?;
    Ok(())
}
