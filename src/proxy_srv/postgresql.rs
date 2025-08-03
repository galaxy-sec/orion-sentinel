//! PostgreSQL 代理实现

use tokio_postgres::{Config, Client, NoTls};
use crate::protocol::sql;

/// PostgreSQL 代理结构体
pub struct PostgreSQLProxy {
    // PostgreSQL 连接配置
    config: PostgreSQLConfig,
}

/// PostgreSQL 配置
pub struct PostgreSQLConfig {
    /// 主机地址
    pub host: String,
    /// 端口号
    pub port: u16,
    /// 数据库名称
    pub database: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

impl PostgreSQLConfig {
    /// 创建 PostgreSQL 连接配置
    pub fn new(host: &str, port: u16, database: &str, username: &str, password: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            database: database.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_postgresql_config_creation() {
        let config = PostgreSQLConfig::new("localhost", 5432, "testdb", "user", "password");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.database, "testdb");
        assert_eq!(config.username, "user");
        assert_eq!(config.password, "password");
    }
    
    #[tokio::test]
    async fn test_execute_query_with_condition() {
        let config = PostgreSQLConfig::new("localhost", 5432, "testdb", "user", "password");
        let proxy = PostgreSQLProxy::new(config);
        
        let query = "SELECT * FROM users";
        let condition = "age > 18";
        let result = proxy.execute_query(query, Some(condition)).await.unwrap();
        assert_eq!(result, "SELECT * FROM users WHERE age > 18");
    }
    
    #[tokio::test]
    async fn test_execute_query_without_condition() {
        let config = PostgreSQLConfig::new("localhost", 5432, "testdb", "user", "password");
        let proxy = PostgreSQLProxy::new(config);
        
        let query = "SELECT * FROM users";
        let result = proxy.execute_query(query, None).await.unwrap();
        assert_eq!(result, "SELECT * FROM users");
    }
}

impl PostgreSQLProxy {
    /// 创建新的 PostgreSQL 代理实例
    pub fn new(config: PostgreSQLConfig) -> Self {
        Self { config }
    }

    /// 连接到 PostgreSQL 数据库
    pub async fn connect(&self) -> Result<Client, Box<dyn std::error::Error>> {
        let mut cfg = Config::new();
        cfg.host(&self.config.host);
        cfg.port(self.config.port);
        cfg.dbname(&self.config.database);
        cfg.user(&self.config.username);
        cfg.password(&self.config.password);
        
        let (client, connection) = cfg.connect(NoTls).await?;
        
        // 在后台处理连接
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        
        Ok(client)
    }

    /// 执行 SQL 查询
    pub async fn execute_query(&self, query: &str, condition: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
        // 如果提供了条件，则添加到查询中
        let final_query = if let Some(condition) = condition {
            sql::add_where_condition(query, condition)?
        } else {
            query.to_string()
        };
        
        println!("Executing query: {}", final_query);
        Ok(final_query)
    }
}