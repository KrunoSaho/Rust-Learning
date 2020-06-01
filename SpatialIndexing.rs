struct Cell {
    idx: i64,
    x: i32,
    y: i32,
}

struct CellIndirector
{
    idx: i64,
    subcells: Vec<Cell>,
}


impl std::fmt::Debug for Cell {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(x:{}, y:{}, idx:{})", self.x, self.y, self.idx)
    }
}

impl std::clone::Clone for Cell {
    fn clone (&self) -> Cell {
        Cell {
            idx: self.idx,
            x: self.x,
            y: self.y,
        }
    }
}

impl std::fmt::Debug for CellIndirector {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[idx: {} = {:?}]", self.idx, self.subcells)
    }
}


fn main() {
    let _initial_points = [(1, 2), (5, 5), (3, 1)];
    let _cells: Vec<Cell> = _initial_points.iter().map(|(x, y)| Cell{idx: 0, x: *x, y: *y}).collect();
    let top_level_grid = CellIndirector {
        idx: 0,
        subcells: _cells.to_vec(),
    };
    println!("{:?}", top_level_grid);
}

