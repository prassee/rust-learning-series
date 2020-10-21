#[derive(Debug)]
pub struct CsvReader {
    file_path: String,
    curr_line: String,
    next_line: String,
}

impl CsvReader {
    pub fn new(file_path: String) -> Self {
        CsvReader {
            file_path,
            curr_line: String::from(""),
            next_line: String::from(""),
        }
    }
}

impl Iterator for CsvReader {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr_line = String::from("");
        self.next_line = String::from("");
        Some(String::from(""))
    }
}
