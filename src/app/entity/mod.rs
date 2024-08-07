pub mod table;
pub mod art;
pub mod attach;
pub mod cate;
pub mod comment;
pub mod page;
pub mod setting;
pub mod tag;
pub mod user;
pub mod guestbook;
pub mod friendlink;

#[inline]
pub fn default<T: Default>() -> T {
    std::default::Default::default()
}
