use serde_derive::{Deserialize, Serialize};

use crate::traits::TestSubAble;
use crate::protocol::sql;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct UserDAcls {
    pub user_id: String,
    pub data_acls: Vec<DataAcl>,
}
impl UserDAcls {
    pub(crate) fn set_user_id<S: Into<String>>(&mut self, user_id: S) {
        self.user_id = user_id.into();
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct DataAcl {
    pub data_view: DataView,
    pub view_scope: ViewScope,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct DataView {
    pub name: String,
    pub source: String,
    #[serde(rename = "USER")]
    pub user_field: String, // 避免与用户身份关键字冲突
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct ViewScope {
    #[serde(rename = "cond")]
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Condition {
    #[serde(rename = "where")]
    pub filter_clauses: Vec<String>, // 避免使用 Rust 关键字
}

impl Condition {
    /// 将所有filter_clauses合并为一个SQL WHERE条件
    pub fn to_sql_where_condition(&self) -> Result<String, Box<dyn std::error::Error>> {
        if self.filter_clauses.is_empty() {
            return Ok(String::new());
        }
        
        // 如果只有一个条件，直接返回
        if self.filter_clauses.len() == 1 {
            return Ok(self.filter_clauses[0].clone());
        }
        
        // 如果有多个条件，使用AND连接
        let mut result = self.filter_clauses[0].clone();
        for clause in &self.filter_clauses[1..] {
            result = format!("{} AND {}", result, clause);
        }
        Ok(result)
    }
    
    /// 将SQL查询与条件合并
    pub fn apply_to_query(&self, sql: &str) -> Result<String, Box<dyn std::error::Error>> {
        let condition = self.to_sql_where_condition()?;
        if condition.is_empty() {
            return Ok(sql.to_string());
        }
        sql::add_where_condition(sql, &condition)
    }
}

impl TestSubAble for UserDAcls {
    fn stub() -> Self {
        UserDAcls {
            user_id: "1001".to_string(),
            data_acls: vec![DataAcl::stub()],
        }
    }
}
impl TestSubAble for DataAcl {
    fn stub() -> Self {
        DataAcl {
            data_view: DataView::stub(),
            view_scope: ViewScope::stub(),
        }
    }
}
impl TestSubAble for DataView {
    fn stub() -> Self {
        DataView {
            name: "test_view".to_string(),
            source: "test_source".to_string(),
            user_field: "user_id".to_string(),
        }
    }
}
impl TestSubAble for ViewScope {
    fn stub() -> Self {
        ViewScope {
            conditions: vec![Condition::stub()],
        }
    }
}
impl TestSubAble for Condition {
    fn stub() -> Self {
        Condition {
            filter_clauses: vec!["status = 'active'".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data_acl::define::UserDAcls;

    #[test]
    fn test_acl_config_structure() -> Result<(), Box<dyn std::error::Error>> {
        let config = UserDAcls {
            user_id: "1".to_string(),
            data_acls: vec![DataAcl {
                data_view: DataView {
                    name: "test_view".to_string(),
                    source: "test_source".to_string(),
                    user_field: "user_id".to_string(),
                },
                view_scope: ViewScope {
                    conditions: vec![Condition {
                        filter_clauses: vec!["status = 'active'".to_string()],
                    }],
                },
            }],
        };

        let toml = toml::to_string(&config)?;
        println!("{}", toml);
        assert!(toml.contains("DATA_ACLS"));
        assert!(toml.contains("USER_ID"));
        Ok(())
    }

    #[test]
    fn test_user_acl_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let data_acl = DataAcl {
            data_view: DataView {
                name: "inventory".into(),
                source: "stock_table".into(),
                user_field: "owner_id".into(),
            },
            view_scope: ViewScope {
                conditions: vec![Condition {
                    filter_clauses: vec!["quantity > 100".into()],
                }],
            },
        };

        let toml = toml::to_string(&data_acl)?;
        println!("{}", toml);
        assert!(toml.contains("NAME = \"inventory\""));
        assert!(toml.contains("[[VIEW_SCOPE.cond]]"));
        Ok(())
    }

    #[test]
    fn test_data_view_mapping() -> Result<(), Box<dyn std::error::Error>> {
        let data_view = DataView {
            name: "sales_view".into(),
            source: "sales_data".into(),
            user_field: "sales_rep".into(),
        };

        let toml = toml::to_string(&data_view)?;
        println!("{}", toml);
        assert!(toml.contains("SOURCE = \"sales_data\""));
        assert!(toml.contains("USER = \"sales_rep\""));
        Ok(())
    }

    #[test]
    fn test_view_scope_conditions() -> Result<(), Box<dyn std::error::Error>> {
        let scope = ViewScope {
            conditions: vec![Condition {
                filter_clauses: vec!["price > 99.99".into(), "currency = 'USD'".into()],
            }],
        };

        let toml = toml::to_string(&scope)?;
        println!("{}", toml);
        assert!(toml.contains("where = [\"price > 99.99\", \"currency = 'USD'\"]"));
        Ok(())
    }

    #[test]
    fn test_condition_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let condition = Condition {
            filter_clauses: vec!["is_valid = true".into(), "created_at > '2023-01-01'".into()],
        };

        let toml = toml::to_string(&condition)?;
        println!("{}", toml);
        assert!(toml.contains("where = [\"is_valid = true\", \"created_at > '2023-01-01'\"]"));
        Ok(())
    }

    #[test]
    fn test_condition_to_sql_where_condition() -> Result<(), Box<dyn std::error::Error>> {
        // 测试单个条件
        let condition = Condition {
            filter_clauses: vec!["status = 'active'".to_string()],
        };
        assert_eq!(condition.to_sql_where_condition()?, "status = 'active'");
        
        // 测试多个条件
        let condition = Condition {
            filter_clauses: vec!["status = 'active'".to_string(), "age > 18".to_string()],
        };
        assert_eq!(condition.to_sql_where_condition()?, "status = 'active' AND age > 18");
        
        // 测试空条件
        let condition = Condition {
            filter_clauses: vec![],
        };
        assert_eq!(condition.to_sql_where_condition()?, "");
        
        Ok(())
    }
    
    #[test]
    fn test_condition_apply_to_query() -> Result<(), Box<dyn std::error::Error>> {
        let condition = Condition {
            filter_clauses: vec!["status = 'active'".to_string(), "age > 18".to_string()],
        };
        
        let sql = "SELECT * FROM users";
        let result = condition.apply_to_query(sql)?;
        assert_eq!(result, "SELECT * FROM users WHERE status = 'active' AND age > 18");
        
        let sql = "SELECT * FROM users WHERE gender = 'male'";
        let result = condition.apply_to_query(sql)?;
        assert_eq!(result, "SELECT * FROM users WHERE gender = 'male' AND status = 'active' AND age > 18");
        
        Ok(())
    }
    
    // 新增多格式解析测试
    #[test]
    fn test_multi_format_parsing() -> Result<(), Box<dyn std::error::Error>> {
        // TOML解析
        let toml_str = r#"
        USER_ID = "json_test"
        [[DATA_ACLS]]
        [DATA_ACLS.DATA_VIEW]
        NAME = "json_view"
        SOURCE = "json_source"
        USER = "json_user"

        [DATA_ACLS.VIEW_SCOPE]
        [[DATA_ACLS.VIEW_SCOPE.cond]]
        where = [ "modified_at > '2023-06-01'" ]
        "#;

        // JSON解析
        let json_str = r#"{
            "USER_ID": "json_test",
            "DATA_ACLS": [{
                "DATA_VIEW": {
                    "NAME": "json_view",
                    "SOURCE": "json_source",
                    "USER": "json_user"
                },
                "VIEW_SCOPE": {
                    "cond": [{
                        "where": [
                            "modified_at > '2023-06-01'"
                        ]
                    }]
                }
            }]
        }"#;

        let toml_config: UserDAcls = toml::from_str(toml_str)?;
        let json_config: UserDAcls = serde_json::from_str(json_str)?;
        assert_eq!(toml_config, json_config);

        Ok(())
    }
}
