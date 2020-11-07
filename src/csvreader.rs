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

// the below code is with multiple lifetime params

#[derive(Debug)]
pub struct CsvLineParser<'lin, 'lim> {
    line: &'lin String,
    delim: &'lim String,
}

impl<'lin, 'lim> CsvLineParser<'lin, 'lim> {
    pub fn new(line: &'lin String, delim: &'lim String) -> Self {
        Self { line, delim }
    }
}

impl<'lin, 'lim> Iterator for CsvLineParser<'lin, 'lim> {
    type Item = Vec<String>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
