use std::io::{self, prelude::*};
use std::{collections::HashMap, error::Error};
use std::{unreachable, vec, writeln};

type Position = usize;
type Value = usize;
#[derive(Debug, Clone)]
struct Mask {
    ones: Vec<usize>,
    xs: Vec<usize>,
}

impl Mask {
    fn new() -> Self {
        Self {
            ones: Vec::new(),
            xs: Vec::new(),
        }
    }

    fn from_str(input: &str) -> Self {
        let len = input.len() - 1;
        let input_iter = input
            .chars()
            .enumerate()
            .filter(|&(_, ch)| ch != '0')
            .map(|(idx, ch)| (len - idx, ch));

        let mut ones: Vec<usize> = vec![];
        let mut xs: Vec<usize> = vec![];

        for (idx, ch) in input_iter {
            match ch {
                'X' => xs.push(idx),
                '1' => ones.push(idx),
                _ => unreachable!(),
            }
        }

        Self { ones, xs }
    }

    fn apply(&self, value: usize) -> Vec<usize> {
        let mut vec_result: Vec<usize> = vec![];
        let mut result = value;
        for &i in self.ones.iter() {
            result = change_bit(result, i, 1);
        }
        // generating all possible permutations for positions with X's
        for permutation in 0..(1 << self.xs.len()) {
            let mut val = result;
            for i in 0..self.xs.len() {
                // going through bits of permutation by shifting to the right and selecting
                // the last bit.
                val = change_bit(val, self.xs[i], (permutation >> i) & 1);
            }
            vec_result.push(val);
        }
        vec_result
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    MASK(Mask),
    MEM(Position, Value),
}

impl Instruction {
    fn from_str(line: &str) -> Self {
        let mut line_iter = line.split(" = ");
        let op = line_iter.next().unwrap().trim();
        let value = line_iter.next().unwrap().trim();

        match op {
            "mask" => Instruction::MASK(Mask::from_str(value)),
            _ => Instruction::MEM(
                op.get(4..op.len() - 1).unwrap().parse::<usize>().unwrap(),
                value.parse::<usize>().unwrap(),
            ),
        }
    }
}

fn change_bit(number: usize, position: usize, new_val: usize) -> usize {
    let mask = 1 << position;
    (number & !mask) | (new_val << position)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let instructions: Vec<Instruction> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| Instruction::from_str(&line))
        .collect();

    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut global_mask: &Mask = &Mask::new();
    for instruction in instructions.iter() {
        match instruction {
            Instruction::MASK(mask) => global_mask = mask,
            &Instruction::MEM(position, value) => {
                for floating in global_mask.apply(position) {
                    // println!("floating= {:?}", floating);
                    memory.insert(floating, value);
                }
            }
        };
    }
    // mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    // mem[8] = 11

    // writeln!(&mut out, "instructions=  {:?}", instructions)?;
    // writeln!(&mut out, "memory= {:?}", memory)?;
    writeln!(&mut out, "sum= {:?}", memory.values().sum::<usize>())?;
    writeln!(&mut out, "memory length= {:?}", memory.len())?;
    Ok(())
}
