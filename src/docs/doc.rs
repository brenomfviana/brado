pub struct Document {
    document: String,
}

impl Document {
    pub fn new(document: &str) -> Document {
        Document { document: document.to_string() }
    }

    pub fn len(&self) -> usize {
        return self.document.chars().count();
    }

    pub fn chars(&self) -> Vec<char> {
        return self.document.chars().collect();
    }

    pub fn get_char(&self, index: usize) -> char {
        return self.document.chars().nth(index).unwrap();
    }
}
