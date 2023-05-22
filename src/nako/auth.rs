extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};

// 加密秘密
pub fn password_hash(password: &str) -> String {
    let hashed: String = hash(password, DEFAULT_COST).unwrap_or("".to_string());

    hashed
}

// 验证密码
pub fn password_verify(password: &str, hash: &str) -> bool {
    let res: bool = verify(password, hash).unwrap_or(false);

    res
}
