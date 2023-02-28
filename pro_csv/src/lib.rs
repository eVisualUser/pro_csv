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
        file.write_all(String::new().as_bytes())?;
        file.write_all(self.to_string().as_bytes())
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

    pub fn get_with_headers(&self, x: usize, y: usize) -> Option<String> {
        self.get(x + 1, y + 1)
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

    pub fn get_longer_line(&self) -> usize {
        let mut longer = 0_usize;
        for line in &self.buffer {
            if line.len() > longer {
                longer = line.len();
            }
        }
        return longer;
    }
}

impl CSV {
    pub fn set(&mut self, line: usize, column: usize, element: String) {
        match self.buffer.get_mut(line) {
            Some(buffer_line) => match buffer_line.get_mut(column) {
                Some(buffer_element) => {
                    *buffer_element = element;
                }
                None => {
                    self.append_column(String::new());
                    self.correct_size();
                    return self.set(line, column, element);
                }
            },
            None => {
                self.append_line(vec![]);
                self.correct_size();
                return self.set(line, column, element);
            }
        }
    }

    pub fn insert_line(&mut self, index: usize, mut line: Vec<String>) {
        for _ in line.len()..self.get_column_count() {
            line.push(String::new());
        }
        self.buffer.insert(index, line);
    }

    pub fn append_line(&mut self, mut line: Vec<String>) {
        for _ in line.len()..self.get_column_count() {
            line.push(String::new());
        }
        self.buffer.push(line);
    }

    pub fn insert_column(&mut self, index: usize, value: String) {
        for line in self.buffer.iter_mut() {
            line.insert(index, value.clone());
        }
    }

    pub fn append_column(&mut self, value: String) {
        for line in self.buffer.iter_mut() {
            line.push(value.clone());
        }
    }

    pub fn resize(&mut self, columns: usize, lines: usize) {
        let mut new_buffer = Vec::<Vec<String>>::new();
        for line in &self.buffer {
            let mut new_line = Vec::<String>::new();
            for element in 0..line.len() {
                if element <= columns {
                    new_line.push(line[element].clone());
                }
            }
            for _ in line.len()..columns {
                new_line.push(String::new());
            }
            new_buffer.push(new_line);
        }
        for _ in self.buffer.len()..lines {
            let mut line = Vec::<String>::new();
            for _ in 0..columns {
                line.push(String::new());
            }
            self.buffer.push(line);
        }
        self.buffer = new_buffer;
    }

    pub fn correct_size(&mut self) {
        let longer_line = self.get_longer_line();
        let lines = self.get_line_count();
        self.resize(longer_line, lines);
    }
}

impl CSV {
    pub fn swap_lines(&mut self, line_a: usize, line_b: usize) -> Result<(), String> {
        let line_count = self.get_line_count();
        if line_a > line_count || line_b > line_count {
            return Err(format!(
                "One of the two line are out of bounds: A({}) B({}), csv size: {:?}",
                line_a,
                line_b,
                self.get_size()
            ));
        }

        let line_a_content = self.get_all_element_of_line(line_a);
        let line_b_content = self.get_all_element_of_line(line_b);
        self.buffer[line_a] = line_b_content;
        self.buffer[line_b] = line_a_content;

        Ok(())
    }

    pub fn swap_columns(&mut self, column_a: usize, column_b: usize) -> Result<(), String> {
        let column_count = self.get_column_count();
        if column_a > column_count || column_b > column_count {
            return Err(format!(
                "One of the two column are out of bounds: A({}) B({}), csv size: {:?}",
                column_a,
                column_b,
                self.get_size()
            ));
        }

        for line in self.buffer.iter_mut() {
            let element_a = line[column_a].clone();
            let element_b = line[column_b].clone();
            line[column_a] = element_b;
            line[column_b] = element_a;
        }

        Ok(())
    }
}
