use std::io::Write;
#[derive(Debug, Clone)]
pub struct CSV {
    separator: char,
    buffer: Vec<Vec<String>>,
}

impl Default for CSV {
    fn default() -> Self {
        Self {
            separator: ';',
            buffer: Vec::new(),
        }
    }
}

impl CSV {
    pub fn set_sperator_char(&mut self, separator: char) {
        self.separator = separator;
    }

    pub fn get_sperator_char(&self) -> char {
        self.separator
    }
}

impl CSV {
    pub fn load(&mut self, content: &str) {
        let mut lines = Vec::<String>::new();
        let mut line = String::new();
        for i in content.chars() {
            if i != '\n' {
                line.push(i);
            } else if !line.is_empty() {
                lines.push(line);
                line = String::new();
            }
        }
        if !line.is_empty() {
            lines.push(line);
        }

        for line in lines.iter() {
            self.buffer.push(self.parse_line(&line));
        }
    }

    pub fn parse_line(&self, line: &str) -> Vec<String> {
        let mut result = Vec::<String>::new();

        let mut buffer = String::new();
        for i in line.chars() {
            if i != self.separator {
                buffer.push(i);
            } else if !buffer.is_empty() {
                result.push(buffer);
                buffer = String::new();
            }
        }
        if !buffer.is_empty() {
            result.push(buffer);
        }

        result
    }
}

impl CSV {
    pub fn load_from_file(&mut self, filename: &str) {
        let file = std::fs::read_to_string(filename).unwrap();
        self.load(&file);
    }
}

impl ToString for CSV {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for line in self.buffer.iter() {
            for i in line.iter() {
                result.push_str(&i);
                result.push(self.get_sperator_char());
            }
            result.push('\n');
        }

        result
    }
}

impl CSV {
    pub fn save<P>(&self, filename: P) -> std::io::Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .open(filename)
            .unwrap();

        write!(&mut file, "{}", self.to_string())
    }
}

impl CSV {
    pub fn find_columns_index_with_name(&self, name: &str) -> Vec<usize> {
        let mut result = Vec::<usize>::new();

        match self.buffer.first() {
            Some(line) => {
                for column in 0..line.len() {
                    if line[column] == name {
                        result.push(column);
                    }
                }
            }
            None => (),
        }

        result
    }

    pub fn find_column_index_with_name(&self, name: &str) -> Option<usize> {
        match self.buffer.first() {
            Some(line) => {
                for column in 0..line.len() {
                    if line[column] == name {
                        return Some(column);
                    }
                }
            }
            None => (),
        };
        None
    }

    pub fn get_all_element_of_column(&self, column: usize) -> Vec<String> {
        let mut result = Vec::<String>::new();

        for line in &self.buffer {
            match line.get(column) {
                Some(element) => result.push(element.clone()),
                None => (),
            }
        }

        result
    }
}

impl CSV {
    pub fn find_lines_index_with_name(&self, name: &str) -> Vec<usize> {
        let mut result = Vec::<usize>::new();

        for line in 0..self.buffer.len() {
            match self.buffer[line].first() {
                Some(line_name) => {
                    if line_name == name {
                        result.push(line);
                    }
                }
                None => (),
            }
        }

        result
    }

    pub fn find_line_index_with_name(&self, name: &str) -> Option<usize> {
        for line in 0..self.buffer.len() {
            match self.buffer[line].first() {
                Some(line_name) => {
                    if line_name == name {
                        return Some(line);
                    }
                }
                None => (),
            }
        }
        None
    }

    pub fn get_all_element_of_line(&self, line: usize) -> Vec<String> {
        let mut result = Vec::<String>::new();

        match self.buffer.get(line) {
            Some(line) => {
                result = line.clone();
            }
            None => (),
        }

        return result;
    }
}

impl CSV {
    pub fn get(&self, x: usize, y: usize) -> Option<String> {
        match self.buffer.get(y) {
            Some(line) => match line.get(x) {
                Some(element) => return Some(element.clone()),
                None => (),
            },
            None => (),
        }

        None
    }

    pub fn get_all(&self) -> Vec<Vec<String>> {
        self.buffer.clone()
    }

    pub fn get_line_count(&self) -> usize {
        self.buffer.len()
    }

    pub fn get_column_count(&self) -> usize {
        match self.buffer.first() {
            Some(line) => line.len(),
            None => 0,
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.get_line_count(), self.get_column_count())
    }
}
