use std::cell::Cell;

use crate::{definitions::{lexer::{Token, TokenKind}, Ast, BinOpAssociativity, BinOpKind, BinOperator, Body, ElseBranch, Expr, ExprId,
        FuncDeclParameter, FunctionReturnTypeSyntax, Item, ItemKind, StaticTypeAnnotation, Stmt,
        StmtId, UnOpKind, UnOperator,
    },
    compilation_unit::{resolve_type_from_string, GlobalScope},
    diagnostics::DiagnosticsBagCell,
    typings::Type,
};

#[Derive(Debug, Clone)]

pub struct Counter{
    value: Cell<usize>,
}

impl Counter{
    pub fn new() -> Self{
        Self{
            value: Cell::new(0),
        }
    }

    pub fn increment(&self){
        let current_value = self.value.get();
        self.value.set(current_value + 1);
    }

    pub fn get_value(&self) -> usize{
        self.value.get()
    }
}

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: Counter,
    diagnostics_bag: DiagnosticsBagCell,
    ast: &'a mut Ast,
    global_scope: &'a mut GlobalScope,
}

impl<'a> Parser<'a>{
    pub fn new(tokens: Vec<Token>,diagnostics_bag: DiagnosticsBagCell,ast: &'a mut Ast,global_scope: &'a GlobalScope,) -> Self{
        Self{
            tokens: tokens.iter().filter(|token| token.kind != TokenKind: Whitespace).map(|token| token.clone()).collect(),
            current: Counter::new(),
            diagnostics_bag,
            ast,
            global_scope,
        }
    }

    pub fn parse(&mut self){
        while let Some(_) = self.next_item().map(|stmt| stmt.id) {}
    }

    fn next_item(&mut self){
        if self.is_at_end(){
            return None;
        }
        Some(self.parse_item())
    }

    fn is_at_end(&self) -> bool{
        self.current().kind == TokenKind::Eof
    }

    fn parse_item(&mut self) -> &Item{
        return match &self.current().kind{
            TokenKind::Func => self.parse_func_item(),
            _ => {
                let id = self.parse_statement();
                self.ast.item_from_kind(ItemID::Stmt(id))
            }
        };

    }

    fn parse_func_item(&mut self) -> &Item{
        let func_keyword = self.consume_and_check(TokenKind::Func).clone();
        let identifier = self.consume_and_check(TokenKind::Identifier).clone();
        let parameters = self.consume_and_check(TokenKind::Parameters).clone();
        let return_type = self.consume_and_check(TokenKind::Return).clone();
        let opening_brace = self.consume_and_check(TokenKind::OpenBrace).clone();
        let mut body = Vec::new();
        while  self.current().kind != TokenKind::CloseBrace && !self.is_at_end(){
            body.push(self.parse_statement());
        }
        let closing_brace = self.consume_and_check(TokenKind::CloseBrace).clone();
    }
}
