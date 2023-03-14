#[macro_export]
macro_rules! init_parser {
    (@munch) => {
        12
    };

    (TokenType = $token:ident; $($name:ident ::= $($($match_part:expr),* => |$($match_as:tt),*| { $func:expr })|+)*) => {
        pub enum ParseItem<'a> {
            Terminal($token<'a>),

            $($name),*
        }
    };
}