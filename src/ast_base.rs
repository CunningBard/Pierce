use crate::lexer::Token;

enum YaplTypes{

}

pub struct MathExpression {

}

pub struct YaplVariable {
    value: i128,
    name: String,
    tokens: Vec<Token>
}

pub enum YAPL {
    Integer(YaplInt)
}