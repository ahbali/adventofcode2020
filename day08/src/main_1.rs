use std::{
    collections::HashSet,
    io::{self, prelude::*},
};
use std::{error::Error, writeln};

#[derive(Copy, Clone)]
enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl Instruction {
    fn from((variant, value): (String, isize)) -> Self {
        match variant.as_str() {
            "acc" => Instruction::ACC(value),
            "jmp" => Instruction::JMP(value),
            "nop" => Instruction::NOP(value),
            _ => unreachable!("instruction not implemented"),
        }
    }
}
#[derive(Clone)]
struct Console {
    boot_code: Vec<Instruction>,
    stack_pointer: usize,
    accumulator: isize,
    visited_indices: HashSet<usize>,
}

impl Iterator for Console {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        // self.next_instruction();
        if self.stack_pointer <= self.boot_code.len() {
            self.next_instruction();
            return Some(self.accumulator);
        }
        None
    }
}

impl Console {
    fn new() -> Self {
        Console {
            boot_code: Vec::<Instruction>::new(),
            stack_pointer: 0,
            accumulator: 0,
            visited_indices: HashSet::<usize>::new(),
        }
    }

    fn load_boot_code(&mut self, boot_code: Vec<Instruction>) {
        self.boot_code = boot_code;
    }

    fn next_instruction(&mut self) {
        let instruction = self.boot_code[self.stack_pointer];
        match instruction {
            Instruction::ACC(arg) => {
                self.accumulator += arg;
                self.stack_pointer += 1;
            }
            Instruction::JMP(arg) => {
                self.stack_pointer = (self.stack_pointer as isize + arg) as usize
            }
            Instruction::NOP(_) => self.stack_pointer += 1,
        }
    }

    fn detect_loops(&mut self) -> Option<isize> {
        while !self.visited_indices.contains(&self.stack_pointer) {
            if self.stack_pointer >= self.boot_code.len() {
                return None;
            }
            self.visited_indices.insert(self.stack_pointer);
            self.next_instruction();
        }
        Some(self.accumulator)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input = reader.lines().filter_map(Result::ok);

    let mut boot_code = Vec::<Instruction>::new();

    for line in input {
        let mut line_iter = line.trim().split_whitespace().take(2);
        let operation = line_iter.next().unwrap().trim().to_string();
        let argument = line_iter.next().unwrap().trim().parse::<isize>()?;
        let instruction = Instruction::from((operation, argument));
        boot_code.push(instruction);
    }

    let mut my_console = Console::new();
    my_console.load_boot_code(boot_code);
    // for i in my_console {}

    let result = my_console.detect_loops();

    match result {
        Some(k) => writeln!(&mut out, " there is a loop!! Accumulator = {:?}", k)?,
        None => writeln!(&mut out, "no loops good boot code")?,
    }

    Ok(())
}
