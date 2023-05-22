pub mod user;
pub mod attach;

#[inline]
pub fn default<T: Default>() -> T {
    std::default::Default::default()
}
