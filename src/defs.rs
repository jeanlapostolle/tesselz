use std::collections::HashMap;

pub type FunctionType = fn(Vec<Object>) -> Object;
pub type SetType = Vec<Object>;

#[derive(Debug, Clone)]
pub enum Object {
    Element(ElementType),
    Set(SetType),
}

#[derive(Debug, Clone)]
pub enum ElementType {
    Number(f32),
    Vector(f32, f32),
    Point(f32, f32),
    Polygon(Vec<(f32, f32)>),
    Function(FunctionType),
}

#[derive(Debug)]
pub struct FunctionCall {
    pub fun: String,
    pub args: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Number(f32),
    Ident(String),
    Set(Vec<Expression>),
    FunctionCall(FunctionCall),
}

pub struct Context {
    pub objects: HashMap<String, Object>,
}

#[derive(Debug)]
pub struct Assignment {
    pub ident: String,
    pub expr: Expression,
}

#[derive(Debug)]
pub struct Render {
    pub filename: String,
    pub expr: Expression,
}

pub enum Statement {
    Assignment(Assignment),
    Render(Render),
}
