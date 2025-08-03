//! SQL 协议处理模块

use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{SetExpr, Statement, Query, Select, Expr};

/// 解析 SQL 语句并增加 WHERE 条件
pub fn add_where_condition(sql: &str, condition: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, sql)?;
    
    if ast.is_empty() {
        return Err("Empty SQL statement".into());
    }
    
    let mut statement = ast[0].clone();
    
    // 检查是否是 SELECT 语句
    if let Statement::Query(query) = &mut statement {
        add_where_to_query(query, condition)?;
    }
    
    Ok(statement.to_string())
}

/// 为查询添加 WHERE 条件
fn add_where_to_query(query: &mut Query, condition: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dialect = GenericDialect {};
    let mut tokenizer = sqlparser::tokenizer::Tokenizer::new(&dialect, condition);
    let tokens = tokenizer.tokenize()?;
    let mut parser = Parser::new(&dialect).with_tokens(tokens);
    let new_condition = parser.parse_expr()?;
    
    if let SetExpr::Select(select) = &mut *query.body {
        add_where_to_select(select, new_condition);
    }
    
    Ok(())
}

/// 为 SELECT 语句添加 WHERE 条件
fn add_where_to_select(select: &mut Select, new_condition: Expr) {
    match &select.selection {
        Some(existing_condition) => {
        // 如果已存在 WHERE 条件，则使用 AND 连接
        let combined_condition = Expr::BinaryOp {
            left: Box::new(existing_condition.clone()),
            op: sqlparser::ast::BinaryOperator::And,
            right: Box::new(new_condition),
        };
        select.selection = Some(combined_condition);
    },
        None => {
        // 如果不存在 WHERE 条件，则直接添加
        select.selection = Some(new_condition);
    }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_where_condition() {
        let sql = "SELECT * FROM users";
        let condition = "age > 18";
        let result = add_where_condition(sql, condition).unwrap();
        assert_eq!(result, "SELECT * FROM users WHERE age > 18");
    }
    
    #[test]
    fn test_add_where_condition_with_existing_where() {
        let sql = "SELECT * FROM users WHERE name = 'John'";
        let condition = "age > 18";
        let result = add_where_condition(sql, condition).unwrap();
        assert_eq!(result, "SELECT * FROM users WHERE name = 'John' AND age > 18");
    }
}