use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct AclConfig {
    pub user_acls: Vec<UserAcl>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct UserAcl {
    pub user_id: u64,
    pub data_view: DataView,
    pub view_scope: ViewScope,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct DataView {
    pub name: String,
    pub source: String,
    #[serde(rename = "USER")]
    pub user_field: String, // 避免与用户身份关键字冲突
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct ViewScope {
    #[serde(rename = "cond")]
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Condition {
    #[serde(rename = "where")]
    pub filter_clauses: Vec<String>, // 避免使用 Rust 关键字
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data_acl::define::AclConfig;

    #[test]
    fn test_acl_config_structure() -> Result<(), Box<dyn std::error::Error>> {
        let config = AclConfig {
            user_acls: vec![UserAcl {
                user_id: 1001,
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
        assert!(toml.contains("USER_ACLS"));
        assert!(toml.contains("USER_ID = 1001"));
        Ok(())
    }

    #[test]
    fn test_user_acl_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let user_acl = UserAcl {
            user_id: 2002,
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

        let toml = toml::to_string(&user_acl)?;
        println!("{}", toml);
        assert!(toml.contains("USER_ID = 2002"));
        assert!(toml.contains("NAME = \"inventory\""));
        assert!(toml.contains("[[cond]]"));
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

    // 新增多格式解析测试
    #[test]
    fn test_multi_format_parsing() -> Result<(), Box<dyn std::error::Error>> {
        // TOML解析
        let toml_config: AclConfig = toml::from_str(
            r#"
            [[USER_ACLS]]
            USER_ID = 1
            [USER_ACLS.DATA_VIEW]
            NAME = "toml_view"
            SOURCE = "toml_source"
            USER = "toml_user"
            [[USER_ACLS.VIEW_SCOPE.cond]]
                where = ["status = 'active'"]
        "#,
        )?;

        // JSON解析
        let json_config: AclConfig = serde_json::from_str(
            r#"{
            "USER_ACLS": [{
                "USER_ID": 1,
                "DATA_VIEW": {
                    "NAME": "json_view",
                    "SOURCE":"json_source",
                    "USER": "json_user"
                },
                "VIEW_SCOPE": {
                    "cond": [
                        { "where": ["status = 'active'"] }
                    ]
                }
            }]
        }"#,
        )?;

        assert_eq!(toml_config.user_acls[0].data_view.name, "toml_view");
        assert_eq!(json_config.user_acls[0].data_view.name, "json_view");
        Ok(())
    }

    #[test]
    fn example_parse() -> Result<(), Box<dyn std::error::Error>> {
        let toml_str = r#"
            [[USER_ACLS]]
            USER_ID = 1
            [USER_ACLS.DATA_VIEW]
            NAME = "order_view"
            SOURCE = "order_table"
            USER = "order_owner_id"

            [[USER_ACLS.VIEW_SCOPE.cond]]
                where = [
                    "create_time > '2020-01-01'",
                    "order_status NOT IN ('CANCELLED', 'PENDING')"
                ]
            [[USER_ACLS.VIEW_SCOPE.cond]]
                where = [
                    "create_time > '2020-01-01'",
                    "order_status NOT IN ('CANCELLED', 'PENDING')"
                ]
        "#;

        let config: AclConfig = toml::from_str(toml_str)?;
        println!("{:#?}", config);
        let json_dat = serde_json::to_string_pretty(&config)?;
        println!("{}", json_dat);
        let config2: AclConfig = serde_json::from_str(&json_dat)?;
        assert_eq!(config, config2);
        Ok(())
    }
}
