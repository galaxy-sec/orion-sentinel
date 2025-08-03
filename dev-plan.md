
# 开发计划

## 目录结构规划

```
src
├── protocol   // 追加数据控制的协议
│   ├── sql
│   │   └── ...
├── data_acl  //数据控制规则
│   └──...
├── ctrl_src // 数据控制源
│   └── from_file.rs
├── data_end  // 数据后端
│   └──...
├── proxy_srv  // 代理服务 
│   └──...
├── config 
│   └──...
├── app
cargo.toml

```

## TODO List
[x] 选择SQL的解析库，获取AST，并可以增加 WHERE 条件
    支持SQL标准 
    结论：选择 `sqlparser-rs` 库，它支持 ANSI SQL:2011 标准，并能将 SQL 查询转换为抽象语法树（AST）。
[x] 支持PostgreSQL PROXY
    结论：已在 `proxy_srv` 目录下创建 `postgresql.rs` 文件，实现了基本的 PostgreSQL 代理结构，并引入了 `tokio-postgres` 作为依赖项。
[x] 支持PostgreSQL PROXY
    结论：选择 `pgrs` 库，它是 PostgreSQL 的 Rust 客户端库，支持 PROXY 协议。
[]  SQL 解析的添加测试用例
[]  实现 SQL WHERE 条件的增加