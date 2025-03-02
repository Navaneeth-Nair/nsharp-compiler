use crate{::ast{
    AssignExpr,Ast, BinaryExpr, BlockExpr, Body, BoolExpr, CallExpr,Expr, ExprId, ExprKind, FuncDeclaration,IfExpr,
    ItemId,ItemKind, LetStmt, NumberExpr, ParenthesizedExpr, ReturnStmt, Stmt, StmtId,  StmtKind, UnaryExpr, VarExpr,
    WhileStmt 
    }

    text::span::TextSpan,
}


pub trait ASTVisitor{
    fn visit_item(&mut self, ast: &mut Ast, item: ItemId){
        self.visit_item_default(ast, item);
    }

    fn visit_body(&mut self, ast: &mut Ast, body: &Body){
        self.visit_body_default(ast, body)
    }

    fn visit_body_default(&mut self, &mut Ast, body: &Body){
        for stmt in body.iter(){
            self.visit_statement(ast, *stmt)
        }
    }

    fn visit_item_default(&mut self, ast: &mut Ast, item: ItemId){
        
    }
}
