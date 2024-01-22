#[derive(Clone, Copy, Debug )]
pub enum CellState {
    Alive,
    Dead,
    Zombie(i32),
}

#[derive(Clone, Debug )]
pub struct Cell {
    pub state: CellState,
    pub neighbors: Vec<(usize, usize)>,
}

impl Cell {
    pub fn new(state: CellState) -> Self {
        Cell {
            state,
            neighbors: Vec::new(),
        }
    }

    pub fn update(&mut self, board: Vec<Vec<Cell>>) {
        let live_neighbors = self
            .neighbors
            .iter()
            .filter(|(i, j)| {
                board
                    .get(*i)
                    .and_then(|row| row.get(*j))
                    .map_or(false, |cell| matches!(cell.state, CellState::Alive))
            })
            .count();

        match self.state {
            CellState::Alive => {
                if live_neighbors < 2 {
                    self.state = CellState::Zombie(3)
                } else if live_neighbors == 2 || live_neighbors == 3 {
                    self.state = CellState::Alive
                } else if live_neighbors > 3 {
                    self.state = CellState::Zombie(3)
                }
            }
            CellState::Dead => {
                if live_neighbors == 3 {
                    self.state = CellState::Alive
                }
            }
            CellState::Zombie(x) => {
                if live_neighbors == 3 {
                    self.state = CellState::Alive
                } else if x == 0 && live_neighbors != 3 {
                    self.state = CellState::Dead
                } else if live_neighbors != 3 {
                    self.state = CellState::Zombie(x - 1)
                }
            }
        }
    }
}
