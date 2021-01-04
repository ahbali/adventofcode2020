use std::io::{self, prelude::*};
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    writeln,
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input = reader.lines().filter_map(Result::ok).map(|line| {
        line.replace("no other bags", "")
            .replace("bags", "")
            .replace("bag", "")
            .replace("bag", "")
            .replace("contain", "")
            .replace(",", "")
            .replace(".", "")
    });
    let shiny = "shinygold".to_string();

    //light red bags contain 1 bright white bag, 2 muted yellow bags.
    //light red bags contain 1 bright white bag, 2 muted yellow bags.  #no other bags
    //light red  contain 1 bright white bag, 2 muted yellow .          #bags
    //light red  contain 1 bright white , 2 muted yellow .             #bag
    //light red   1 bright white , 2 muted yellow .                    #contain
    //light red   1 bright white  2 muted yellow .                     #,
    //light red   1 bright white  2 muted yellow                       #.
    //light red   1 bright white  2 muted yellow  #

    let mut rules_map: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for line in input {
        let mut line_iter = line.split_whitespace();
        let mut rule: HashMap<String, usize> = HashMap::new();
        let mut buffer = 0;
        let mut key = line_iter.next().unwrap().to_string();
        key.push_str(line_iter.next().unwrap());

        loop {
            // populating the rule line
            let mut next = match line_iter.next() {
                Some(k) => k.to_string(),
                None => break,
            };
            if let Ok(k) = next.parse::<usize>() {
                buffer = k;
            } else {
                next.push_str(line_iter.next().unwrap());
                rule.insert(next, buffer);
            }
        }

        rules_map.insert(key, rule);
    }
    let mut result: usize = 0;

    let mut queue: VecDeque<(&String, usize)> =
        rules_map[&shiny].iter().map(|(k, &v)| (k, v)).collect();

    while queue.len() != 0 {
        let (name, quantity) = queue.pop_front().unwrap();
        result += quantity;
        queue.extend(
            rules_map[name]
                .iter()
                .map(|(key, value)| (key, value * quantity)),
        );
    }

    writeln!(&mut out, "{:?}", result)?;

    Ok(())
}
