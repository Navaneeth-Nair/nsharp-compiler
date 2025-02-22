pub mod span;

pub struct SourceText{
    text:String
}

impl SourceText{
    pub fn new(text:String) -> Self{
        Self{ text }
    }

    pub fn line_index(&self,position: usize) -> usize{
        self.position[..=position].nth(index).unwrap()
    }

    pub fn line_start(&self, position: usize) -> usize{
        self.text
            .lines()
            .take(index)
            .map(|line| line.len + 1)
            .sum()
    }
}