#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub literal: String,
}

impl Span {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}
