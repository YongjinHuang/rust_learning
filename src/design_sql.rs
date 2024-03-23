use std::collections::HashMap;

/**
 * Your SQL object will be instantiated and called as such:
 * let obj = SQL::new(names, columns);
 * obj.insert_row(name, row);
 * obj.delete_row(name, rowId);
 * let ret_3: String = obj.select_cell(name, rowId, columnId);
 */
type RowID = i32;

struct RowIdGenerator {
    row_id: RowID,
}

impl RowIdGenerator {
    fn new() -> Self {
        RowIdGenerator {
            row_id: 0
        }
    }
    fn generate_new_row_id(&mut self) -> RowID {
        self.row_id += 1;
        self.row_id
    }
}

struct Table {
    row_id_generator: RowIdGenerator,
    rows: HashMap<RowID, Vec<String>>,
}

impl Table {
    fn new() -> Self {
        Self {
            row_id_generator: RowIdGenerator::new(),
            rows: HashMap::new(),
        }
    }
    fn insert_row(&mut self, row: Vec<String>) {
        let new_id = self.row_id_generator.generate_new_row_id();
        self.rows.insert(new_id, row);
    }
    fn delete_row(&mut self, row_id: RowID) {
        self.rows.remove(&row_id);
    }
    fn select_cell(&self, row_id: RowID, column_id: i32) -> String {
        let row = self.rows.get(&row_id);
        if let Some(row) = row {
            if let Some(cell) = row.get(column_id as usize) {
                cell.clone()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    }
}


struct SQL {
    tables: HashMap<String, Table>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SQL {
    fn new(names: Vec<String>, columns: Vec<i32>) -> Self {
        let mut tables = HashMap::new();
        for name in names.iter() {
            tables.insert(name.clone(), Table::new());
        }
        SQL {
            tables
        }
    }

    fn insert_row(&mut self, name: String, row: Vec<String>) {
        if let Some(table) = self.tables.get_mut(&name) {
            table.insert_row(row);
        }
    }

    fn delete_row(&mut self, name: String, row_id: i32) {
        if let Some(table) = self.tables.get_mut(&name) {
            table.delete_row(row_id);
        }
    }

    fn select_cell(&self, name: String, row_id: i32, column_id: i32) -> String {
        if let Some(table) = self.tables.get(&name) {
            table.select_cell(row_id, column_id - 1)
        } else {
            "".to_string()
        }
    }
}

