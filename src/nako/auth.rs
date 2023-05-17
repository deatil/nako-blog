extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};

// 加密秘密
pub fn password_hash(password: &str) -> String {
    let hashed: String = match hash(password, DEFAULT_COST) {
        Ok(data) => data,
        Err(_) => "".to_string(),
    };

    hashed
}

// 验证密码
pub fn password_verify(password: &str, hash: &str) -> bool {
    let res: bool = match verify(password, hash) {
        Ok(data) => data,
        Err(_) => false,
    };

    res
}
