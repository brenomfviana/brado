pub struct Document {
    document: String,
}

impl Document {
    pub fn new(document: &str) -> Document {
        Document {
            document: document.to_string(),
        }
    }

    pub fn len(&self) -> usize {
        self.document.chars().count()
    }

    pub fn is_empty(&self) -> bool {
        self.document.chars().count() == 0
    }

    pub fn chars(&self) -> Vec<char> {
        self.document.chars().collect()
    }

    pub fn get_char(
        &self,
        index: usize,
    ) -> char {
        self.document.chars().nth(index).unwrap()
    }
}
