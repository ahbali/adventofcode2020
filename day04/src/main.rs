use std::{
    collections::HashMap,
    io::{self, prelude::*},
};
use std::{error::Error, writeln};

fn is_valid(data: &str) -> bool {
    let tokens = [
        "byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:", //"cid:",
    ];
    for &token in tokens.iter() {
        if !data.contains(token) {
            return false;
        }
    }
    true
}

fn is_data_valid(data: &str) -> bool {
    let passport: HashMap<_, _> = data
        .split_whitespace()
        .map(|token| token.split(':').take(2).collect::<Vec<&str>>())
        .map(|vector| (vector[0], vector[1]))
        .collect();

    println!("{:?}", passport);

    if let Some(&byr) = passport.get("byr") {
        if byr.len() > 4 {
            return false;
        }
        if !(1920..=2002).contains(&byr.parse::<usize>().unwrap()) {
            return false;
        }
    }
    if let Some(&iyr) = passport.get("iyr") {
        if iyr.len() > 4 {
            return false;
        }
        if !(2010..=2020).contains(&iyr.parse::<usize>().unwrap()) {
            return false;
        }
    }
    if let Some(&eyr) = passport.get("eyr") {
        if eyr.len() > 4 {
            return false;
        }
        if !(2020..=2030).contains(&eyr.parse::<usize>().unwrap()) {
            return false;
        }
    }
    if let Some(&hgt) = passport.get("hgt") {
        // let unit = &hgt[(hgt.len() - 2)..];
        // let value = &hgt[..(hgt.len() - 2)];
        let (value, unit) = hgt.split_at(hgt.len() - 2);
        match unit {
            "in" => {
                if !(59..=76).contains(&value.parse::<usize>().unwrap()) {
                    return false;
                }
            }

            "cm" => {
                if !(150..=193).contains(&value.parse::<usize>().unwrap()) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    if let Some(&hcl) = passport.get("hcl") {
        if hcl.len() != 7 {
            return false;
        }
        match hcl.chars().nth(0) {
            Some('#') => {
                for element in hcl.chars().skip(1) {
                    if !('0'..='9').contains(&element) && !('a'..='f').contains(&element) {
                        return false;
                    }
                }
            }
            _ => return false,
        }
    }
    if let Some(&ecl) = passport.get("ecl") {
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl) {
            return false;
        }
    }
    if let Some(&pid) = passport.get("pid") {
        if pid.len() != 9 {
            return false;
        }
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input = reader.lines().filter_map(Result::ok);

    let mut buffer = String::new();
    let mut valid_count = 0;

    for line in input {
        if line.len() == 0 {
            let temp_str = buffer.trim();
            if is_valid(temp_str) {
                if is_data_valid(temp_str) {
                    // writeln!(&mut out, "{:?}", temp_str)?;
                    valid_count += 1;
                }
            }
            buffer.clear();
        }
        buffer.push(' ');
        buffer.push_str(&line);
    }
    // writeln!(&mut out, "last buffer = {:?}", buffer)?;
    // if is_valid(&buffer) {
    //     if is_data_valid(&buffer) {
    //         valid_count += 1;
    //     }
    // }
    buffer.clear();

    writeln!(&mut out, "valid = {:?}", valid_count)?;

    Ok(())
}
