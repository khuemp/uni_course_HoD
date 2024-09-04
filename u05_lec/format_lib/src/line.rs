pub struct Line {
    words: Vec<String>,
    maximum_length: usize,
}

const SEPARATOR: &'static str = " ";
const SEPARATOR_LENGTH: usize = SEPARATOR.len();

impl Line {
    pub fn new(maximum_length: usize) -> Self {
        Self {
            words: vec![],
            maximum_length,
        }
    }

    pub fn try_push(&mut self, word: String) -> Option<String> {
        let current_length: usize = self.words.iter().map(|w| w.len()).sum();
        let current_length_with_separator = current_length + (self.words.len()) * SEPARATOR_LENGTH;
        if current_length_with_separator + SEPARATOR_LENGTH + word.len() <= self.maximum_length {
            self.words.push(word);
            None
        } else {
            Some(word)
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.pad(&format!("{}", self.words.join(SEPARATOR)))
    }
}
