
# 数据访问控制

## 定义

ACL（访问控制列表）由以下三个核心要素组成：
1. **访问者** - 系统用户身份标识
2. **数据视图** - 受控访问的数据集合
3. **视图范围** - 数据访问的过滤条件

### 要素说明
- `USER_ID`：访问者身份标识
- `DATA_VIEW`：被保护的数据表/视图
- `VIEW_USER`：数据属主关联字段
- `VIEW_SCOPE`：基于属主的访问条件

## 配置示例

```toml
USER_ID = "U001"
[[DATA_ACLS]]
[DATA_ACLS.DATA_VIEW]
NAME = "json_view"
SOURCE = "json_source"
USER = "json_user"

[DATA_ACLS.VIEW_SCOPE]
[[DATA_ACLS.VIEW_SCOPE.cond]]
where = [ "modified_at > '2023-06-01'" ]
```
