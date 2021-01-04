use std::error::Error;
use std::io::{self, prelude::*};
use std::writeln;

#[derive(Debug)]
enum Direction {
    NORTH(isize),
    SOUTH(isize),
    EAST(isize),
    WEST(isize),
    LEFT(isize),
    RIGHT(isize),
    FORWARD(isize),
}

impl Direction {
    fn from(k: (Option<char>, Option<isize>)) -> Self {
        match k {
            (Some('N'), Some(number)) => Direction::NORTH(number),
            (Some('S'), Some(number)) => Direction::SOUTH(number),
            (Some('E'), Some(number)) => Direction::EAST(number),
            (Some('W'), Some(number)) => Direction::WEST(number),
            (Some('L'), Some(number)) => Direction::LEFT(number),
            (Some('R'), Some(number)) => Direction::RIGHT(number),
            (Some('F'), Some(number)) => Direction::FORWARD(number),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Ship {
    position_x: isize,
    position_y: isize,
    angle: isize,
}

impl Ship {
    fn new() -> Self {
        Self {
            position_x: 0,
            position_y: 0,
            angle: 0,
        }
    }

    fn goto(&mut self, dir: Direction) {
        // Action N means to move north by the given value.
        // Action S means to move south by the given value.
        // Action E means to move east by the given value.
        // Action W means to move west by the given value.
        // Action L means to turn left the given number of degrees.
        // Action R means to turn right the given number of degrees.
        // Action F means to move forward by the given value in the direction the ship is currently
        // facing.
        match dir {
            Direction::NORTH(number) => self.position_y += number,
            Direction::SOUTH(number) => self.position_y -= number,
            Direction::EAST(number) => self.position_x += number,
            Direction::WEST(number) => self.position_x -= number,
            Direction::LEFT(number) => self.angle = (self.angle + number) % 360,
            Direction::RIGHT(number) => self.angle = (360 + self.angle - number) % 360,
            Direction::FORWARD(number) => self.forward(number),
        }
    }

    fn forward(&mut self, value: isize) {
        match self.angle {
            0 => self.goto(Direction::EAST(value)),
            90 => self.goto(Direction::NORTH(value)),
            180 => self.goto(Direction::WEST(value)),
            270 => self.goto(Direction::SOUTH(value)),
            _ => unreachable!("impossible angle"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let input: Vec<Direction> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            (
                line.trim().chars().nth(0),
                line.trim().get(1..).unwrap().parse::<isize>().ok(),
            )
        })
        .map(|dir| Direction::from(dir))
        .collect();

    // writeln!(&mut out, "input = {:?}", input)?;

    let mut ship = Ship::new();
    for direction in input {
        ship.goto(direction);
    }

    // At the end of these instructions, the ship's Manhattan distance (sum of the absolute values
    //  of its east/west position and its north/south position) from its starting position is
    //  17 + 8 = 25.
    let result = ship.position_x.abs() + ship.position_y.abs();

    writeln!(&mut out, "result = {:?}", result)?;

    Ok(())
}
