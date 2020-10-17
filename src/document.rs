use crate::Position;
use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
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
    pub fn insert(&mut self, at: &Position, c: char) {
				let y_position = at.y;
        if y_position == self.len() {
            let mut row: Row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if y_position < self.len() {
            let row: &mut Row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
		}
		pub fn delete(&mut self, at: &Position) {
			if at.y >= self.len() { 
				return;
			}
			let row = self.rows.get_mut(at.y).unwrap();
			row.delete(at.x);
		}
}
