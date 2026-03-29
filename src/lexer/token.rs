use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token(".")]
    Dot,

    #[token("-")]
    Dash,

    #[token(" ")]
    Space,

    #[regex(r"[a-zA-Z]+")]
    Text,
}
