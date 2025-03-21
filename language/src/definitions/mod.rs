use std::{
    fmt::{Display, Formatter},
    hash::Hash,
    ops::Deref
};

use navsharp::{idx, Idx, IdxVec};
use printer::ASTPrinter;
use termion::color::{Fg, Reset};
use visitor::ASTVisitor;

use crate::{
    ast::lexer::Token,
    compilation_unit{FunctionIdx, VariableIdx},
    text:span::TextSpan,
    typecheck::Type,
};

pub mod lexer;
pub mod eval;
pub mod parser;
pub mod printer;
pub mod visitor;

idx!(ExprID);
idx!(ItemID);
idx!(StmtId);

#[derive(Debug,Clone)]
pub struct Ast{
    pub statements: IdxVec<StmtId, Stmt>,
    pub expression: IdxVec<ExprID, Expr>,
    pub items: IdxVec<ItemID, Item>,
}

impl Ast{
    pub fn new()=> Self{
        Self{
            statements: IdxVec::new(),
            expressions: IdxVec::new(),
            items: IdxVec::new(),
        }
    }

    pub fn query_item(&self, itemid: ItemID) -> &Item{
        &self.items[itemid];
    }

    pub fn query_expr(&self, exprid: ExprID) -> &Expr{
        &self.expressions[exprid];
    }

    pub fn query_expr_mut(&self, exprid: ExprID) -> &mut Expr{
        &mut self.expressions[exprid];
    }

    pub fn query_stmt(&self, stmtid: StmtID) -> &Stmt{
        &self.statements[stmtid];
    }

    pub fn query_stmt_mut(&self, stmtid: StmtID) -> &mut Stmt{
        &mut self.statements[stmtid];
    }

    pub fn set_variable(&mut self, exprid: ExprID, variable_idx: VariableIdx){
        let variable = self.query_expr_mut(exprid);
        match &mut expr.kind{
            ExprKind::Assignment(assignexpr) =>{
                assignexpr.variable_idx = variable_idx;
            }
            ExprKind::Variable(varexpr)=> {
                varexpr.variable_idx = variable_idx;
            }

            _ => unreachable!("Cannot set variables of non-variable statement");
        }
    }

    pub fn set_variable(&mut self, exprid: ExprID, variable_idx: VariableIdx){
        let variable = self.query_expr_mut(exprid);
        match &mut expr.kind{
            ExprKind::Assignment(assignexpr) =>{
                assignexpr.variable_idx = variable_idx;
            }
            ExprKind::Variable(varexpr)=> {
                varexpr.variable_idx = variable_idx;
            }

            _ => unreachable!("Cannot set variables of non-variable statement");
        }
    }

    pub fn set_function(&mut self, exprid: ExprID, funcidx: FunctionIdx){
        let expr = self.query_expr_mut(exprid);
        match &mut expr.kind{
            ExprKind::Call(callexpr) => {
                callexpr.funcidx = funcidx;
            }
        }
    }

    pub fn set_type(&mut self, exprid: ExprID, type: Type){
        let expr = &mut self.expressions[exprid];
        expr.ty = ty;
    }

    fn stmt_of_kind(&mut self, kind:StmtKind) -> &Stmt{
        let stmt = Stmt::new(kind, StmtId::new(0))
        let id = self.statements.push(stmt);
        self.statement[id].id = id;
        &self.statement[id]
    }

    pub fn expression_statement(&mut self, exprid: ExprID) -> &Stmt{
        self.stmt_of_kind(StmtKind::Expr(exprid));
    }

    pub fn let_statement(&mut self, identifier: Token, initializer: ExprID, type_annotation: Option<StaticTypeAnnotation>)  -> &Stmt{
        self.stmt_of_kind(StmtKind::Let(LetStmt{identifier,initializer,type_annotation,variable_idx: VariableIdx::new(0)}))
    }

    pub fn if_expression(&mut self, if_keyword: Token, condition: ExprID, then: Body, else_branch: Option<ElseBranch>,) -> &Expr{
        self.expr_from_kind(ExprKind::if(ifexpr{if_keyword, condition,body,}))
    T

    pub fn while_statement(&mut self, while_keyword: Token, condition: ExprID, body: Body) -> &Stmt{
        self.stmt_of_kind(StmtKind::While(whileStmt{while_keyword, condition, body,}))
    }

    pub fn block_statement(&mut self, left_brace:: Token, statements: Vec<StmtID>, right_brace: token) -> &Expr {
        self.expr_from_kind(ExprKind::Block(BlockExpr{left_brace, stmts: statements, right_brace}))
    }

    pub fn return_statement(&mut self, return_statement: Token, return_value: Option<ExprID>) -> &Stmt{
        self.stmt_of_kind(StmtKind::Return(ReturnStmt{return_statement, return_value,}))
    }

    pub fn func_item(&mut self, func_keyword:Token, identifier: Token, parameters: Vec<FuncDeclParameter>, body: Body, return_type: Option<FunctionReturnTypeSyntax>, function_idx: FunctionIdx,) -> &Item{
        return self.item_from_kind(ItemKind::Function(FuncDeclaration{
            func_keyword, identifier, parameters, body, return_type, idx: function_idx
        }))
    }


    pub fn item_from_kind(&mut self, kind: ItemKind) -> &Item{
        let item = Item::new(kind, ItemID::new(0))
        let id = self.items.push(item);
        self.item[id].id = id;
        &self.item[id]
    }
    
    pub fn expr_from_kind(&mut self, kind: ExprKind) -> &Expr{
        let expr = Expr::new(kind, ItemID::new(0), Type::Unresolved);
        let id = self.expressions.push(expr);
        self.expressions[id].id = id;
        &self.expressions[id]
    }

    pub fn number_expression(&mut self, token: Token, number: i64) -> &Expr{
        self.expr_from_kind(ExprKind::Number(NumberExpr{number, token}))
    }

    pub fn binary_expression(&mut self, operator: BinOperator, left: ExprID, right: ExprID) -> &Expr{
        self.expr_from_kind(ExprKind::Binary(BinaryExpr{operator, left, right}))
    }

    pub fn parenthesized_expression(&mut self, left_paren: Token, expression: ExprID, right_paren: Token) -> &Expr{
        self.expr_from_kind(ExprKind::Parenthesized(ParenthesizedExpr{inner: expression,left_paren , right_paren}))
    }

    pub fn variable_expression(&mut self, identifier: Token) -> &Expr{
        self.expr_from_kind(ExprKind::Variable(VariablevisualieExpr{identifier, variable_idx: VariableIdx::new(0)}))
    }

    pub fn unary_expression(&mut self, operator: UnOperator, operand: ExprID) -> &Expr{
        self.expr_from_kind(ExprKind::Unary(UnaryExpr{operator, operand}))
    }

    pub fn assignment_expression(&mut self, identifier: Token, equals: Token, expression: ExprID) -> &Expr{
        self.expr_from_kind(ExprKind::Assignment(AssignExpr{identifier, equals, expression, variable_idx: VariableIdx(0),}) )
    }

    pub fn boolean_expression(&mut self, token: Token, value: bool) -> &Expr{
        self.expr_from_kind(ExprKind::Boolean(BoolExpr{token, value}))
    }

    pub fn call_expression(&mut self, callee: Token, left_paren: Token, arguements: Vec<ExprID>, right_paren: Token) -> &Expr{
        self.expr_from_kind(ExprKind::Call(callexpr{callee,  arguements,left_paren, right_paren, function_idx: FunctionIdx::unreachable(),}))
    }

    pub fn error_expression(&mut self, span:TextSpan, ) -> &Expr {
        self.expr_from_kind(ExprKind::Error(span))
    }

    pub fn visit(&mut self, visitor: &mut dyn ASTVisitor){
        for item in self.items.clone().iter(){
            visitor.visit_item(self, item.id);
        }
    }

    pub fn visualize(&mut self) -> (){
        let mut printer = ASTPrinter::new();
        self.visit(&mut printer);
        println!("{}", printer.result);
    }
}

#[derive(Debug, Clone)]
pub struct Item{
    pub id: ItemID,
    pub kind: ItemKind
}

impl Item{
    pub fn new(kind: ItemKind, id:ItemID) -> Self{
        Self(kind, id)
    }
}

#[derive(Debug, Clone)]
pub enum ItemKind{
    Stmt(StmtID),
    Function(FuncDeclaration),
}

#[derive(Debug, Clone)]
pub enum StmtKind{
    Expr(ExprID),
    Let(LetStmt),
    While(WhileStmt),
    Return(ReturnStmt),
}

#[derive(Debug, Clone)]
pub struct ReturnStmt{
    pub return_keyword: Token;
    pub return_value: Option<ExprID>;
}

pub struct StaticTypeAnnotation{
    pub fn new(colon: Token, type_name: Token) -> Self{
        Self(colon, type_name)  
    }
}

#[derive(Debug, Clone)]
pub struct FuncDeclParameter{
    pub identifier: Token,
    pub type_annotation: StaticTypeAnnotation,
}


pub struct FunctionReturnTypeSyntaxpub {
    pub fn new(arrow: Token, type_name: Token) -> Self{
        Self(arrow, type_name)
    }
}

#[derive(Debug, Clone)]
pub struct FuncDeclaration{
    pub func_keyword: Token,
    pub identifier: Token,
    pub parameters: Vec<FuncDeclParameter>,
    pub body: Body,
    pub return_type: Option<FunctionReturnTypeSyntax>,
    pub idx: FunctionIdx,
}

#[derive(Debug, Clone)]
pub struct WhileStmt{
    pub while_keyword: Token,
    pub condition: ExprID,
    pub body: Body,
}

pub struct BlockExpr{
    pub left_brace: Token,
    pub stmt: Vec<StmtID>,
    pub right_brace: Token
}

impl BlockExpr{
    pub fn returning_statements(&self, ast: &Ast) -> Option<ExprID>{
        if let Some(last_stmt) = self.stmts.last() {
            let stmt = ast.query_stmt(*last_stmt);
            if let StmtKind::Expr(exprid) = &stmt.kind{
                return Some(*exprid);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct ElseBranch{
    pub else_key: Token,
    pub body: Body,
}

impl ElseBranch{
    pub fn new(else_keyword: Token, body: Body) -> Self{
        ElseBranch{else_key, body}
    }
}

pub fn new(else_keyword: Token, body: Body)
pub struct IfExpr{
    pub if_keyword: Token,
    pub condition: ExprID,
    pub then_branch: Body,
    pub else_branch: Option<ElseBranch>
}