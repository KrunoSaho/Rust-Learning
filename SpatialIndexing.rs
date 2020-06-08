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

impl CellIndirector {
    fn get_position(&self, id: i64) -> Option<&Cell> {
        let node = self;

        // Leaf node (data is only contained in leaf nodes)
        if node.children.is_empty() {
            let found = node.cells.iter().find(|c| c.id == id);

            if found.is_some() {
                return found;
            }
        }

        // Visits all children in a linear fashion
        node.children.iter().find_map(|n| {
            let cell = n.get_position(id);
            if cell.is_some() {
                return cell;
            }
            None
        })
    }

    fn partition_until_bucket_size(&mut self, size: usize) {
        let node = self;

        if node.cells.len() <= size || node.cells.is_empty() {
            return;
        }

        let cells = &mut node.cells;
        cells.sort_by(|u, v| u.x.cmp(&v.x));
        let (left, right) = cells.split_at(size);

        let left_node = CellIndirector {
            idx: node.idx + 1,
            cells: left.to_vec(),
            children: vec![],
        };

        let mut right_node = CellIndirector {
            idx: node.idx + 2,
            cells: right.to_vec(),
            children: vec![],
        };

        right_node.partition_until_bucket_size(size);

        if !left.is_empty() {
            node.children.push(left_node);
        }

        if !right.is_empty() {
            node.children.push(right_node);
        }

        node.cells = vec![];
    }

    fn add_point_to_cell(&mut self, xy_id: (i64, i32, i32)) {
        let cell = Cell {
            id: xy_id.0,
            x: xy_id.1,
            y: xy_id.2,
        };

        self.cells.push(cell);
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
    for y in 0..8 {
        for x in 0..8 {
            root.add_point_to_cell((idx, x, y));
            idx += 1;
        }
    }

    // Query
    println!("{:#?}", root.get_position(25));
    root.partition_until_bucket_size(2);
    println!("{:#?}", root);
}
