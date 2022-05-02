use unicode_segmentation::UnicodeSegmentation;
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            len: 0,
            string: String::from(slice),
        };
        row.update_len();
        row
    }
}

impl Row {
    #[must_use] pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.string.len());
        let start = std::cmp::min(start, end);
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push(' ');
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }

    #[must_use] pub fn len(&self) -> usize {
        self.string[..].graphemes(true).count()
    }

    #[must_use] pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn update_len(&mut self) {
        self.len = self.string.as_str().graphemes(true).count();
    }
}
