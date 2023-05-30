use rand::Rng;

pub fn random_b64(len: usize) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";
    let mut rng = rand::thread_rng();
    let mut s = String::with_capacity(len);
    for _ in 0..len {
        s.push(chars.chars().nth(rng.gen_range(0..64)).unwrap());
    }
    return s;
}
