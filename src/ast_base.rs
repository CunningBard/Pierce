use crate::lexer::Token;

pub enum PierceTypes {
    // Heap
    Int,
    Str,
    Bool,
    Float,
    List,

    // Stack
    Array,
    String,

    I8,
    I16,
    I32,
    I64,
    I128,

    U8,
    U16,
    U32,
    U64,
    U128,

    F32,
    F64
}

pub struct MathExpression {

}

pub struct Variable {
    value: Vec<str>,
    name: String,
    tokens: Vec<Token>
}

pub struct FuncCall {

}

pub enum Pierce {
    Variable(Variable)
}