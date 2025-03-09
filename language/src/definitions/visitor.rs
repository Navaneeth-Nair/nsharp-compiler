use crate{::definitions::{
    AssignExpr, Ast, BinaryExpr, BlockExpr, Body, BoolExpr, CallExpr, Expr, ExprId, ExprKind,
    FuncDeclaration, IfExpr, ItemId, ItemKind, LetStmt, NumberExpr, ParenthesizedExpr,
    ReturnStmt, Stmt, StmtId, StmtKind, UnaryExpr, VarExpr, WhileStmt,
    },
    text::span::TextSpan,
};

pub trait ASTVisitor{
    fn visit_item(&mut self,ast: &mut Ast, item: ItemId){
        self.visit_item_default(ast, item);
    }

    fn visit_body(&mut self, ast: &mut Ast, body: &Body){
        self.visit_body_default(ast, body);
    }

    fn visit_body_default(&mut self, ast: &mut Ast, body: &Body){
        for stmt in body.iter(){
            self.visit_statement(ast, *statement);
        }
    }


    fn visit_item_default(&mut self, ast: Ast, item: &ItemID){
        let item = ast.query_item(item).clone();
        match &item.kind{
            ItemKind::Stmt(stmt) => {self.visit_statement(ast, *statement);}
            ItemKind::Function(func) =>{self.visit_func_decl(ast, func_decl, item.id);}
        }
    }
}

fn visit_func_decl(&mut self, ast: &mut Ast, func_decl: &FuncDeclaration, item_id: ItemID);

fn do_visit_statement(&mut self, ast:&mut Ast, statement: StmtId){
    let statement = stmt.query_stmt(statement).clone();
    match &statement.kind{
        StmtKind::Return(stmt) => {self.visit_statement(ast, &stmt);}
        StmtKind::Let(expr) => {self.visit_let_statement(ast, &statement);}
        StmtKind::While(stmt) => {self.visit_while_statement(ast, &stmt);}
        StmtKind::Expr(expr) => {self.visit_expression(ast, *expr);}
    }
}

fn visit_return_statement(&mut self, ast:&mut Ast, returnstmt: &ReturnStmt){
    if let Some<expr> = &return.return_value{
        self.visit_expression(ast, *expr);
    }
}

fn visit_while_statement(&mut self, ast: &mut Ast, whilestmt: &WhileStmt){
    self.visit_expression(ast, whileStmt.condition);
    self.visit_body(ast, &whileStmt.body);
}

fn visit_block_expr(&mut self, ast: &mut Ast, blockexpr: &BlockExpr, _expr: &Expr){
    for stmt in block_statement.stmts{
        self.visit_statement(ast, *statement);
    }
}

fn visit_if_expression(&mut self, ast: &mut Ast, ifexpr: &IfExpr, _expr: &Expr){
    self.visit_expression(ast, ifexpr.condition);
    for statement in ifexpr.then_branch.iter(){
        self.visit_statement(ast, *statement);
    }

    if let Some(else_branch) = &ifexpr.else_branch{
        for statement in else_branch.body.iter(){
            self.visit_statement(ast, *statement);
        }
    }
}

fn visit_let_statement(&mut self, ast: &mut Ast, letstmt: &LetStmt, stmt: &Stmt);
fn visit_statement(&mut self, ast: &mut Ast, stmt: StmtId){
    self.do_visit_statement(ast, statement);
}

fn do_visit_expression(&mut self, ast: &mut Ast, expression: &ExprID){
    let expression = expression.query_expr(expression).clone();
    match &expression.kind{
        ExprKind::Number(number) =>{self.visit_number_expression(ast, number, &expression);}
        ExprKind::Binary(expr) =>{self.visit_binary_expression(ast, expr, &expression);}
        ExprKind::Parentheshized(expr) =>{self.visit_number_expression(ast, expr, &expression);}
        ExprKind::Error(span) =>{self.visit_error(ast, span);}
        ExprKind::variable(expr) =>{self.visit_variable_expression(ast, expr, &expression);}
        ExprKind::Unary(expr) =>{self.visit_unary_expression(ast, expr, &expression);}
        ExprKind::Assignment(expr) =>{self.visit_assignment_expression(ast, expr, &expression);}
        ExprKind::Call(expr) => {self.visit_call_expression(ast,expr, &expression)}
        ExprKind::Boolean(expr) => {self.visit_boolean_expression(ast,expr, &expression)}
        ExprKind::If(expr) => {self.visit_if_expression(ast,expr, &expression)}
        ExprKind::Block(expr) => {self.visit_block_expression(ast,expr, &expression)}
    }
}

fn visit_call_expression(&mut self, ast: &mut Ast, callexpr: &CallExpr, _expr: &Expr){
    for argument in call_expression.arguments{
        self.visit_expression(ast, *expression)
    }
}

fn visit_expression(&mut self, ast: &mut Ast, expression: &ExprID){
    self.do_visit_expression(ast, expression);
}

fn visit_assignment_expression(&mut self, ast: &mut Ast, assignexpr: &AssignExpr, _expr: &Expr){
    self.visit_expression(ast, assignment_expression.expression)
}

fn visit_variable_expression(&mut self, ast: &mut Ast, varexpr: &VarExpr, _expr: &Expr);

fn visit_number_expression(&mut self, ast: &mut Ast, numberexpr: &NumberExpr, _expr: &Expr);

fn visit_boolean_expression(&mut self, ast: &mut Ast, boolexpr: &BoolExpr, _expr: &Expr);

fn visit_unary_expression(&mut self, ast: &mut Ast, unary_expression: &UnaryExpr, _expr: &Expr);

fn visit_binary_expression(&mut self, ast: &mut Ast, binexpr: &BinaryExpr, _expr: &Expr){
    self.visit_expression(ast, binary_expression.left);
    self.visit.expression(ast, binary_expression.right)
}

fn visit_parenthesized_expression(^&mut self, ast: &mut Ast, parenthesizedexpr: &ParenthesizedExpr, _expr: &Expr){
    self.visit_expression(ast, parenthesized_expression.inner)
}