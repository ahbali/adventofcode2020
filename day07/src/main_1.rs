use std::{collections::HashMap, error::Error, writeln};
use std::{
    collections::HashSet,
    io::{self, prelude::*},
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
    // let mut result = 0;
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

        // populating Rules Map
        // writeln!(&mut out, "{:?}", rule)?;
        rules_map.insert(key, rule);
    }

    let keys = rules_map.keys().collect::<Vec<_>>();
    let mut positive: HashSet<&String> = HashSet::new();
    let mut visited: HashSet<&String> = HashSet::new();
    positive.insert(&shiny);

    for key in keys {
        let mut children: Vec<&String> = rules_map[key].keys().collect();
        visited.insert(key);
        while children.len() != 0 {
            if let Some(_) = children.iter().find(|&&element| positive.contains(element)) {
                positive.insert(key);
                break;
            };
            let temp_child = children.pop().unwrap();
            if !visited.contains(temp_child) {
                children.extend(rules_map[temp_child].keys());
            };
        }
    }

    writeln!(&mut out, "{:?}", positive.len() - 1)?;
    writeln!(&mut out, "{:#?}", rules_map.len())?;
    // writeln!(&mut out, "{:#?}", visited)?;

    Ok(())
}
