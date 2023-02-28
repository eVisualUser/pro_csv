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
