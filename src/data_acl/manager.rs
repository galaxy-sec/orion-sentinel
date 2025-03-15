use crate::data_acl::define::UserDAcls;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AclManager {
    configs: Arc<Mutex<HashMap<String, UserDAcls>>>,
}

impl AclManager {
    pub fn new() -> Self {
        AclManager {
            configs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_config(&self, user_id: String, config: UserDAcls) {
        let mut configs = self.configs.lock().unwrap();
        configs.insert(user_id, config);
    }

    pub fn get_config(&self, user_id: &str) -> Option<UserDAcls> {
        let configs = self.configs.lock().unwrap();
        configs.get(user_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::TestSubAble;

    use super::*;
    use std::thread;

    #[test]
    fn test_concurrent_access() {
        let manager = AclManager::new();
        let test_user_id = 1;
        let prototype_config = UserDAcls::stub();

        // 并发写入
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let manager = manager.clone();
                let mut acl = prototype_config.clone();
                let user_id = test_user_id + i;
                acl.set_user_id(user_id.to_string());
                thread::spawn(move || {
                    manager.register_config(user_id.to_string(), acl);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        // 并发读取
        let read_handles: Vec<_> = (0..10)
            .map(|i| {
                let manager = manager.clone();
                let user_id = test_user_id + i;
                thread::spawn(move || {
                    assert!(manager.get_config(user_id.to_string().as_str()).is_some());
                })
            })
            .collect();

        for handle in read_handles {
            handle.join().unwrap();
        }
    }
}
