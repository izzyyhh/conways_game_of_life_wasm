use crate::cell::{Cell, CellState};

pub fn intialize_board(board_size: usize) -> Vec<Vec<Cell>> {
    let mut board: Vec<Vec<Cell>> = Vec::new();

    for i in 0..board_size {
        let mut row: Vec<Cell> = Vec::new();
        for j in 0..board_size {
            let state = if (i + j) % 2 == 0 {
                CellState::Alive
            } else {
                CellState::Dead
            };
            row.push(Cell::new(state));
        }
        board.push(row);
    }

    board
}

pub fn build_neighbors(board: &mut Vec<Vec<Cell>>, board_size: usize) {
    for i in 0..board_size {
        for j in 0..board_size {
            board[i][j].neighbors.clear();

            for x in (i as isize - 1)..=(i as isize + 1) {
                for y in (j as isize - 1)..=(j as isize + 1) {
                    if x >= 0
                        && y >= 0
                        && x < board_size as isize
                        && y < board_size as isize
                        && !(x == i as isize && y == j as isize)
                    {
                        board[i][j].neighbors.push((x as usize, y as usize));
                    }
                }
            }
        }
    }
}