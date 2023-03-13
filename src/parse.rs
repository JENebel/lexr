#[derive(Debug, PartialEq)]
pub enum ParseItem<'a, T> {
    NonTerminal(&'a str),
    Terminal(&'a T),
}



#[macro_export]
macro_rules! init_parser {
    () => {
        
    };
}