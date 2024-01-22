#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
  Alive, 
  Dead,
  Zombie(i32)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub state: CellState,
    pub live_neighbors: usize,
}

impl Cell {
     pub fn new(state: CellState) -> Self {
        Cell {
            state,
            live_neighbors: 0,
        }
    }
}
