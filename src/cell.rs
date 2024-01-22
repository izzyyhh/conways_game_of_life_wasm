#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
    Zombie(i32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub state: CellState,
    pub live_neighbors: usize,
    pub neighbors: Vec<(usize, usize)>,
}

impl Cell {
    pub fn new(state: CellState) -> Self {
        Cell {
            state,
            live_neighbors: 0,
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
              if live_neighbors > 0 && live_neighbors != 3 {
                self.state = CellState::Zombie(x -1)
              } else if live_neighbors == 3 {
                self.state = CellState::Alive
              } else if x == 0 && live_neighbors != 3 {
                self.state = CellState::Dead
              }
            }
        }
    }
}
