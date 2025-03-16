use std::collections::HashMap;

use navsharp::Idx;

use crate::{
    definitions::{
        visitor::ASTVisitor, AssignExpr, Ast, BinOpKind, BinaryExpr, BlockExpr, Body, BoolExpr,
        CallExpr, Expr, FuncDeclaration, IfExpr, ItemId, LetStmt, NumberExpr,
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

impl<'a> ASTVisitor for ASTEval<'a>{
    fn visit_body(&mut self,&mut ast, body: &Body){
        self.push_frame();
        for statement in body.iter(){
            self.visit_statement(ast, *statement);
        }
        self.pop_frame();
    }

    fn visit_func_decl(&mut self, _ast: &mut Ast, _func_declaration: &FuncDeclaration, _item_ID: ItemId){

    }
    
} 

fn visit_while_statement(&mut self, ast: &mut Ast, while_statement: &WhileStmt){
    self.push_frame();
    self.visit_statement(ast, while_statement.condition);
    while self.expect_last_value().expect_boolean(){
        self.visit_body(ast, &while_statement.body)
        self.visit_expression(ast, while_statement.condition)
    }
    self.pop_frame();
}

fn visit_block_expr(&mut self, ast: &mut Ast, block_statement: &BlockExpr, _expr: &Expr){
    self.push_frame();
    for statement in block_statement.stmts{
        self.visit_statement(ast, *statement)
    }
    self.pop_frame();
}

fn visit_if_expression(&mut self, ast: &Ast, ifexpr: &IfExpr, _expr: &Expr){
    self.push_frame();
    self.visit_statement(ast, *if_statement.condition)
    if self.expect_last_value().expect_boolean(){
        self.push_frame();
        for statement in if_statement.then_branch.iter(){
            self.visit_statement(ast, *statement);
        }
        self.pop_frame();
    }
    else if let Some(else_branch)  = &if_statement.else_branch{
        self.push_frame();
        for statement in else_branch.body.iter(){
            self.visit_statement(ast, *statement);
        }
        self.pop_frame();
    }      
    self.pop();
}

fn visit_let_statement(&mut self, ast: &Ast, letstmt: &LetStmt, _expr: &Expr){
    self.visit_statement(ast, *let_statement.initializer);
    self.frames.insert(let_statement.variable_idx, self.expect_last_value());
}

fn visit_call_expression(&mut self, ast: &Ast, callexpr: &CallExpr, _expr: &Expr){
    let function_name = call_expression.function_name();
    let function = self.global_scope.lookup_function(funtion_name).map(|f| self.global_scope.get(f)).expect(format!("Function {} not found", function_name).as_str());
    let mut arguments = Vec::new();
    for argument in &call_expression.arguments{
        self.visit_statement(ast, *statement)
        argument.push(self.last_value.unwrap());
    }
    self.push_frame();
    for (argument, param) in arguments.iter().zip(function.parameters.iter()){
        self.frames.insert(*param, *argument);
    }

    for stmt in &*function.body{
        self.visit_statement(ast, *stmt);
    }
    self.pop_frame();
}

fn visit_assignment_expression(&mut self, ast: &Ast, assignexpr: &AssignExpr, _expr: &Expr){
    self.visit_statement(ast, assignexpr.expression);
    self.frames.update(assignexpr.variable_idx  , self.last_value.unwrap());
}

fn visit_variable_expression(&mut self, _ast: &Ast, varexpr: &VarExpr, _expr: &Expr){
    let identifier = &varexpr.identifier.span.literal;
    self.last_value = Some(
        *self.frames.get(&varexpr.variable_idx).expect(format!
            ("Variables {} "{}" not found", varexpr.variable_idx.as_index(), identifier).as_str()
    ),
);
}

fn visit_number_expression(&mut self, _ast: &Ast, numberexpr: &NumberExpr, _expr: &Expr){
    self.last_value = Some(Value::Number(number.number));
}

fn visit_boolean_expression(&mut self, _ast: &Ast, boolexpr: &BoolExpr, _expr: &Expr){
    self.last_value = Some(Value::Boolean(boolean.value));
}

fn visit_error(&mut self, _ast: &Ast, _span: &TextSpan){
    panic!("Cannot Eval an Error Expression!")
}

fn visit_unary_expression(&mut self, ast: &Ast, uexpr: &UnaryExpr, _expr: &Expr){
    self.visit_expression(ast, unary_expression.operand);
    self.last_value = Some(Value::Number(match unary_expression.operand.kind{
        UnOpKind::Minus => -operand,
        UnOpKind::Bitwise => !operand,
    }
)
);
}

fn visit_binary_expression(&mut self, ast: &Ast, binexpr: &BinaryExpr, _expr: &Expr){
    self.visit_expression(ast, binexpr.left);
    let left = self.expect_last_value();
    self.visit_expression(ast, binexpr.right);
    let right= self.expect_last_value();
    self.last_value = Some(match binexpr.operator.kind{
        BinOpKind::Plus =>  Value::Number(left.expect_number() + right.expect_number()),
        BinOpKind::Minus =>  Value::Number(left.expect_number() - right.expect_number()),
        BinOpKind::Multiply =>  Value::Number(left.expect_number() * right.expect_number()),
        BinOpKind::Divide =>  Value::Number(left.expect_number() / right.expect_number()),
        BinOpKind::Modulo =>  Value::Number(left.expect_number() % right.expect_number()),
        BinOpKind::BitwiseAnd =>  Value::Number(left.expect_number() & right.expect_number()),
        BinOpKind::BitwiseOr =>  Value::Number(left.expect_number() | right.expect_number()),
        BinOpKind::BitwiseXor =>  Value::Number(left.expect_number() ^ right.expect_number()),
        
        BinOpKind::Power =>  Value::{Number(left.expect_number().pow(right.expect_number()as u32))}

        BinOpKind::Equalsto =>  Value::Boolean(left == right),
        BinOpKind::NotEqualsto =>  Value::Boolean(left != right),
        BinOpKind::LessThan =>  Value::Boolean(left.expect_number() < right.expect_number()),
        BinOpKind::GreaterThan =>  Value::Boolean(left.expect_number() > right.expect_number()),
        BinOpKind::LessThanEqualTo =>  {
            Value::Boolean(left.expect_number() <= right.expect_number()),
        }
        BinOpKind::GreaterThanEqualTo =>  {
            Value::Boolean(left.expect_number() >= right.expect_number()),
        }
    });
}

fn visit_parenthesized_expression(&mut self, ast: &mut Ast, parenthesizedexpr: &ParenthesizedExpr, _expr: &Expr){
    self.visit_expression(ast, parenthesizedexpr.inner);
}