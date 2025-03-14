
#  How to Develop

## Code Structure

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