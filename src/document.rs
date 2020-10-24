use crate::Position;
use crate::Row;
use std::io::{Error, Write};
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
}

impl Document {
    pub(crate) fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents: String = fs::read_to_string(filename)?;
        let mut rows: Vec<Row> = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
        })
    }

    #[must_use] pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use] pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use] pub fn len(&self) -> usize {
        self.rows.len()
    }
    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.rows.len() {
            return;
        }
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
        }
        let new_row = self.rows[at.y].split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        let y_position = at.y;
        if y_position == self.rows.len() {
            let mut row: Row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else  {
            let row: &mut Row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        self.dirty = false;
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.rows.len();

        if at.y >= len { 
            return;
        }
        self.dirty = true;
        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row: Row = self.rows.remove(at.y + 1);
            let row: &mut Row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
                let row = &mut self.rows[at.y];
                row.delete(at.x);
            }
    }

    pub fn new_line(&mut self, at: &Position) {
        let row: Row = Row::default();
        self.rows.insert(at.y + 1, row);
    }
}
