use matcher::Pattern;
use parsers::Parser;
use matcher::result::MatchResult;

pub trait SuffixArray: Clone {
    fn new() -> Self;
    fn insert(&mut self, pattern: Pattern);
}

pub trait Entry {
    type SA: SuffixArray;
    fn pattern(&self) -> Option<&Pattern>;
    fn set_pattern(&mut self, pattern: Option<Pattern>);
    fn child(&self) -> Option<&Self::SA>;
    fn child_mut(&mut self) -> Option<&mut Self::SA>;
    fn set_child(&mut self, child: Option<Self::SA>);
    fn insert(&mut self, pattern: Pattern) {
        if pattern.pattern().is_empty() {
            self.set_pattern(Some(pattern));
        }
        else {
            if self.child().is_none() {
                let sa = Self::SA::new();
                self.set_child(Some(sa));
            }

            self.child_mut().expect("Failed to get a child").insert(pattern);
        }
    }
}

pub trait LiteralEntry: Entry + Clone {
    fn literal(&self) -> &String;
}

pub trait ParserEntry: Entry + Clone {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<MatchResult<'a, 'b>>;
    fn parser(&self) -> &Box<Parser>;
}
