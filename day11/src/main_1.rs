use std::error::Error;
use std::io::{self, prelude::*};
use std::{vec, writeln};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Empty,
    Occupied,
    Floor,
}

fn neighbor_cells(i: usize, j: usize, board: &Vec<Vec<Cell>>) -> Vec<&Cell> {
    let l_len = board[0].len() as isize;
    let c_len = board.len() as isize;
    let i = i as isize;
    let j = j as isize;
    let neighbors = vec![
        (i, j - 1),
        (i, j + 1),
        (i - 1, j),
        (i + 1, j),
        (i - 1, j - 1),
        (i + 1, j + 1),
        (i - 1, j + 1),
        (i + 1, j - 1),
    ];
    neighbors
        .iter()
        .filter_map(|&cell| match cell {
            (l, c) if l < 0 || c < 0 || l >= l_len || c >= c_len => Some(&Cell::Floor),
            (l, c) => Some(&board[c as usize][l as usize]),
        })
        .collect()
}

fn transform_board(board: &mut Vec<Vec<Cell>>, shadow_board: &mut Vec<Vec<Cell>>) {
    let line_length = board[0].len();
    let col_length = board.len();

    while board != shadow_board {
        for j in 0..col_length {
            for i in 0..line_length {
                let neighbors = neighbor_cells(i, j, &board);
                let occupied_seats = neighbors
                    .iter()
                    .filter(|&&&cell| cell == Cell::Occupied)
                    .count();

                shadow_board[j][i] = match board[j][i] {
                    Cell::Empty if occupied_seats == 0 => Cell::Occupied,
                    Cell::Occupied if occupied_seats >= 4 => Cell::Empty,
                    k => k,
                }
            }
        }
        board.swap_with_slice(shadow_board);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let reader = io::BufReader::new(stdin.lock());

    let mut board: Vec<Vec<Cell>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.trim()
                .chars()
                .filter_map(|c| match c {
                    'L' => Some(Cell::Empty),
                    '.' => Some(Cell::Floor),
                    _ => None,
                })
                .collect::<Vec<Cell>>()
        })
        .collect();

    let mut shadow_board = board.clone();
    shadow_board[0][0] = Cell::Occupied;

    transform_board(&mut board, &mut shadow_board);

    let result: usize = board
        .iter()
        .map(|v| v.iter().filter(|&&cell| cell == Cell::Occupied).count())
        .sum();

    writeln!(&mut out, "result = {:?}", result)?;

    // writeln!(&mut out, "neighbor cells = {:?}", v)?;

    Ok(())
}
