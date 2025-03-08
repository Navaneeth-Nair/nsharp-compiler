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
    ast::lexer::token,
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

    pub fn let_statement(&mut self, identifier: token, initializer: ExprID, type_annotation: Option<StaticTypeAnnotation>)  -> &Stmt{
        self.stmt_of_kind(StmtKind::Let(LetStmt{identifier,initializer,type_annotation,variable_idx: VariableIdx::new(0)}))
    }

    
}