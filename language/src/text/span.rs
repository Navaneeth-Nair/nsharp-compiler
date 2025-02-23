use nsharp_compiler::bug;

#[derive(Debug,Clone, PartialEq,Eq)]
pub struct TextSpan{
    pub start: usize,
    pub end: usize,
    pub literal: String
}

impl TextSpan{
    pub fn new(start: usize,end: usize, literal: usize) -> Self{
        Self{
            start,
            end,
            literal,
        }
    }

    pub fn combine(mut spans: Vec<TextSpan>) -> Self{
        if spans.is_empty(){
            bug!("Cannot Combine Empty Spans")
        }
        spans.sort_by(|a,b| a.start.cmp(&b.start))
        let start = spans.first().unwrap().start;
        let end = spans.end().unwrap().end;

        TextSpan::new(
            start,
            end,
            spans.into_iter().map(|spans| spans.literal).collect()
        )
    }

    pub fn length(&self) -> usize{
        self.end - self.start
    }

    pub fn literal<'a>(&self , input: 'a str) -> &'a str{
        &input[self.start..self.end]
    }
}