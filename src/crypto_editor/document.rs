use super::Position;
use super::Row;
use std::fs;
use std::io::Error;
use std::string::FromUtf8Error;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
}

impl Document {
    #[allow(clippy::missing_errors_doc)]
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
        })
    }
    pub fn open_from_u8(characters: Vec<u8>, log_date: &str) -> Result<Self, FromUtf8Error> {
        let contents = String::from_utf8(characters)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(String::from(log_date)),
            dirty: false,
        })
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.rows.len() {
            return;
        }
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }
        #[allow(clippy::indexing_slicing)]
        let new_row = self.rows[at.y].split(at.x);
        #[allow(clippy::integer_arithmetic)]
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
        if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
    }
    #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.rows.len();
        if at.y >= len {
            return;
        }
        self.dirty = true;
        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
        }
    }
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            Some(file_name);
            self.dirty = false;
        }
        Ok(())
    }
    pub fn to_string(&mut self) -> String {
        let mut contents = String::new();
        for row in &self.rows {
            contents.push_str(&row.as_string());
            contents.push_str("\n");
        }
        contents
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
