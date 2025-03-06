use std::collections::HashMap;

use navsharp::Idx;

use crate::{
    ast::{
        visitor::ASTVisitor, AssignExpr, Ast, BinOpKind, BinaryExpr, BlockExpr, Body, BoolExpr,
        CallExpr, Expr, FunctionDeclaration, IfExpr, ItemId, LetStmt, NumberExpr,
        ParenthesizedExpr, Stmt, UnOpKind, UnaryExpr, VarExpr, WhileStmt,
    },
    compilation_unit::{FunctionIdx, GlobalScope, VariableIdx},
    text::span::TextSpan,
};


#[derive(Debug)]

pub struct Frame{
    variables: HashMap<VariableIdx, Value>
}

impl Frame{
    fn new() -> Self{
        Self{
            variables: HashMap::new(),
        }
    }

    fn insert(&mut self, idx: VariableIdx, value: Value){
        self.variables.insert(idx, value);
    }

    fn get(&self , idx: &VariableIdx) -> Option<Value>{
        self.variables.get(idx)
    }
}

#[derive(Debug)]

pub struct Frames{
    Frames: Vec<Frame>,
}

impl Frames{
    fn new() -> Self{
        Self{
            frames: vec![Frame::new()],
        }
    }

    fn push(&mut self){
        self.frames.push(Frame::new());
    }

    fn pop(&mut self){
        self.frames.pop();
    }

    fn update(&mut self, idx: VariableIdx, value: Value){
        for frame in self.frames.iter_mut().rev(){
            if frame.get(&idx).is_some(){
                frame.insert(idx, value);
                return;
            }
        }
    }

    fn insert (&mut self, idx: VariableIdx, value: Value){
        self.frames.last_mut().unwrap().insert(idx, value);
    }

    fn get(&self, idx: &VariableIdx) -> Option<&Value>{
        for frame in self.frame.iter().rev(){
            if let Some(value) = frame.get(idx){
                return Some(value);
            }
        }
        None
    }
}

#[derive[Debug,Clone,Copy,PartialEq]]

pub enum Value{
    Number(i64),
    Boolean(bool),
    Function(FunctionIdx),
}

impl Value{
    pub fn expect_boolean(&self) -> bool{
        match self{
            Value::Boolean(value) => *value,
            _ => panic!("Expected a boolean expression"),   
        }
    }

    pub fn expect_number(&self) -> i64 {
        match self {
            Value::Number(value) => *value,
            _ => panic!("Expected a Integer Expression"),
        }
    }

    pub fn expect_function(&self) -> FunctionIdx{
        match self {
            Value::Function(value) => *value,
            _ => panic!("Expexted a Function Expression")
        }
    }
}

pub struct ASTEval<'a>{
    pub last_value: Option<Value>,
    pub frames : Frames,
    pub global_scope: &'a GlobalScope
}

impl<'a> ASTEval<'a>{
    pub fn new(global_scope: &'a GlobalScope) -> Self{
        Self{
            last_value: None,
            frames: Frames::new(),
            global_scope,  
        }
    }

    fn push_frame(&mut self){
        self.frames.push();

    }

    fn pop_frame(&mut self){
        self.frames.pop();
    }

    fn expect_last_value(&self) -> Value{
        *self
        .last_value
        .as_ref()
        .expect("expected last value to be set")
    }
}