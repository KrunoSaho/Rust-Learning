
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

fn add_point_to_cell(xy_id: (i64, i32, i32), points: &mut Vec<Cell>) {
    let cell = Cell{ id: xy_id.0, x: xy_id.1, y: xy_id.2 };
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


fn get_position(node: &CellIndirector, id: i64) -> Option<&Cell> {
    if node.children.is_empty() {
        let found = node.cells.iter().find(|c| c.id == id);

        if found.is_some() {
            return found;
        }
    }

    let res = node.children.iter()
        .find(|n| get_position(n, id).is_some());

    if res.is_none() {
        None
    }
    else {
        get_position(res.unwrap(), id)
    }
}


fn partition_until_bucket_size(node: &mut CellIndirector, size: usize) {
    if node.cells.len() <= size {
        return;
    }

    let cells = &node.cells;
    let (left, right) = cells.split_at(size);

    let left_node = CellIndirector {
        idx: node.idx + 1,
        cells: left.to_vec(),
        children: vec![],
    };

    let mut right_node = CellIndirector {
        idx: node.idx + 2,
        cells: right.to_vec(),
        children: vec![]
    };

    partition_until_bucket_size(&mut right_node, size);

    node.children.append(vec![left_node].as_mut());
    node.children.append(vec![right_node].as_mut());
}


/********************************** Run *******************************/

fn main() {
    let mut main_cells = vec![];

    // Add data
    let mut idx: i64 = 0;
    for y in 0..10 {
        for x in 0..10 {
            add_point_to_cell((idx, x, y), &mut main_cells);
            idx += 1;
        }
    }

    // Add overlord grid
    let mut root = CellIndirector {
        idx: 0,
        cells: main_cells,
        children: vec![],
    };

    // Query
    println!("{:?}", get_position(&root, 99));
    partition_until_bucket_size(&mut root, 5);
    println!("{:?}", root);
}
