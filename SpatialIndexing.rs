

/********************************** Data *******************************/

struct Cell {
    id: i64,
    x: i32,
    y: i32,
}

struct CellIndirector
{
    idx: i64,
    cells: Vec<Cell>,
    children: Vec<CellIndirector>,
}


/********************************** Data formatting ********************/
impl std::fmt::Debug for Cell {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(x:{}, y:{}, id:{})", self.x, self.y, self.id)
    }
}

impl std::clone::Clone for Cell {
    fn clone (&self) -> Cell {
        Cell {
            id: self.id,
            x: self.x,
            y: self.y,
        }
    }
}

impl std::fmt::Debug for CellIndirector {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[idx: {} = {:?}], [{:?}]", self.idx, self.cells, self.children)
    }
}


/********************************** Functions *******************************/

fn add_point_to_cell(xy: (i32, i32), points: &mut Vec<Cell>) {
    let cell = Cell{ id: 0, x: xy.0, y: xy.1 };
    points.push(cell);
}


fn partition_cells(overlord: &CellIndirector) -> CellIndirector {
    let mut new_cells = overlord.cells.to_vec();
    new_cells.sort_by(|c, d| c.x.cmp(&d.x));

    let left = CellIndirector {
        idx: overlord.idx + 1,
        cells: new_cells.drain(0..new_cells.len() / 2).collect(),
        children: vec![],
    };

    let right = CellIndirector {
        idx: overlord.idx + 2,
        cells: new_cells.to_vec(),
        children: vec![],
    };

    CellIndirector {
        idx: overlord.idx,
        cells: vec![],
        children: vec![left, right],
    }
}


/********************************** Run *******************************/

fn main() {
    let mut main_cells = vec![];

    // Add data
    [(1, 2), (5, 5), (3, 1), (5, 3)]
        .iter()
        .for_each(|x| add_point_to_cell(*x, &mut main_cells));

    // Add overlord grid
    let root = CellIndirector {
        idx: 0,
        cells: main_cells,
        children: vec![],
    };

    println!("{:?}", root);
    let new_root = partition_cells(&root);
    println!("{:?}", new_root);
}
