/********************************** Data *******************************/

#[derive(Debug, Clone)]
struct Cell {
    id: i64,
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct CellIndirector {
    idx: i64,
    cells: Vec<Cell>,
    children: Vec<CellIndirector>,
}

/********************************** Impl *******************************/

impl From<(i64, i32, i32)> for Cell {
    fn from ((id, x, y): (i64, i32, i32)) -> Self {
        Self {id, x, y}
    }
}


impl CellIndirector {
    fn get_position(&self, id: i64) -> Option<&Cell> {
        // Leaf node (data is only contained in leaf nodes)
        if self.children.is_empty() {
            self.cells.iter().find(|c| c.id == id)
        }
        else {
            // Visits all children in a linear fashion
            self.children.iter().find_map(|n| n.get_position(id))
        }
    }

    fn partition_until_bucket_size(&mut self, size: usize) {
        if self.cells.len() <= size {
            return;
        }

        let cells = &mut self.cells;
        cells.sort_by(|u, v| u.x.cmp(&v.x).then_with(|| u.y.cmp(&v.y)));
        let (left, right) = cells.split_at(size);

        let left_node = CellIndirector {
            idx: self.idx + 1,
            cells: left.to_vec(),
            children: vec![],
        };

        let mut right_node = CellIndirector {
            idx: self.idx + 2,
            cells: right.to_vec(),
            children: vec![],
        };

        right_node.partition_until_bucket_size(size);

        if !left.is_empty() {
            self.children.push(left_node);
        }

        if !right.is_empty() {
            self.children.push(right_node);
        }

        self.cells.clear();
    }

    fn add_point(&mut self, point: (i64, i32, i32)) {
        self.cells.push(point.into());
    }
}

/********************************** Run *******************************/

fn main() {
    let mut root = CellIndirector {
        idx: 0,
        cells: vec![],
        children: vec![],
    };

    // Add data
    let mut idx: i64 = 0;
    for y in 0..3 {
        for x in 0..3 {
            root.add_point((idx, x, y));
            idx += 1;
        }
    }

    // Query
    println!("{:?}", root.get_position(6));
    root.partition_until_bucket_size(1);
    println!("{:?}", root);
}
