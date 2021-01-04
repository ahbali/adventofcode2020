use std::error::Error;
use std::{
    cmp::max,
    io::{self, prelude::*},
};
use std::{vec, writeln};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Empty,
    Occupied,
    Floor,
}

fn neighbor_cells(i: usize, j: usize, board: &Vec<Vec<Cell>>) -> Vec<Option<&Cell>> {
    let l_len = board[0].len();
    let c_len = board.len();

    let max = max(c_len, l_len);

    let line_plus = (1..l_len)
        .filter(|k| i + k < l_len)
        .filter(|&k| board[j][i + k] != Cell::Floor)
        .map(|k| &board[j][i + k])
        .next();

    let line_minus = (1..l_len)
        .filter(|&k| (i as isize - k as isize) >= 0)
        .filter(|&k| board[j][i - k] != Cell::Floor)
        .map(|k| &board[j][i - k])
        .next();

    let col_plus = (1..c_len)
        .filter(|k| j + k < c_len)
        .filter(|&k| board[j + k][i] != Cell::Floor)
        .map(|k| &board[j + k][i])
        .next();

    let col_minus = (1..c_len)
        .filter(|&k| (j as isize - k as isize) >= 0)
        .filter(|&k| board[j - k][i] != Cell::Floor)
        .map(|k| &board[j - k][i])
        .next();

    let r_diag_plus = (1..max)
        .filter(|k| j + k < c_len && i + k < l_len)
        .filter(|&k| board[j + k][i + k] != Cell::Floor)
        .map(|k| &board[j + k][i + k])
        .next();

    let r_diag_minus = (1..max)
        .filter(|&k| (j as isize - k as isize) >= 0 && (i as isize - k as isize) >= 0)
        .filter(|&k| board[j - k][i - k] != Cell::Floor)
        .map(|k| &board[j - k][i - k])
        .next();

    let l_diag_plus = (1..max)
        .filter(|&k| (j as isize - k as isize) >= 0 && i + k < l_len)
        .filter(|&k| board[j - k][i + k] != Cell::Floor)
        .map(|k| &board[j - k][i + k])
        .next();

    let l_diag_minus = (1..max)
        .filter(|&k| j + k < c_len && (i as isize - k as isize) >= 0)
        .filter(|&k| board[j + k][i - k] != Cell::Floor)
        .map(|k| &board[j + k][i - k])
        .next();
    vec![
        line_plus,
        line_minus,
        col_plus,
        col_minus,
        r_diag_plus,
        r_diag_minus,
        l_diag_plus,
        l_diag_minus,
    ]
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
                    .filter(|&&cell| cell == Some(&Cell::Occupied))
                    .count();

                shadow_board[j][i] = match board[j][i] {
                    Cell::Empty if occupied_seats == 0 => Cell::Occupied,
                    Cell::Occupied if occupied_seats >= 5 => Cell::Empty,
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

    Ok(())
}
