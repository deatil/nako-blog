use std::path::Path;
use std::ffi::OsStr;

use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::digest::Digest;
use uuid::Uuid;
use humansize::{format_size, DECIMAL};

// md5
pub fn md5(data: &str) -> String {
    let mut h = Md5::new();
    h.input_str(data);
    h.result_str()
}

// sha1
pub fn sha1(data: &str) -> String {
    let mut h = Sha1::new();
    h.input_str(data);
    h.result_str()
}

// sha1
pub fn hmac_sha1<'a>(
    data: &'a str, 
    key: &'a str,
) -> String {
    let mut hmac = Hmac::new(Sha1::new(), &key.as_bytes());

    hmac.input(&data.as_bytes());

    let binding = hmac.result();
    let code = binding.code();

    let s = String::from_utf8_lossy(code);

    s.to_string()
}

// uuid
pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn get_extension(filename: &str) -> String {
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);

    if let Some(ext) = extension {
        return ext.to_string();
    }

    "".to_string()
}

pub fn get_path_filename(path: &str) -> String {
    let path_string = path.to_string();
    let files = path_string.split("/").collect::<Vec<_>>();
    let file_name = files.last();

    match file_name {
        Some(v) => v.to_string(),
        None => "".to_string(),
    }
}

pub fn formatsize<T: humansize::ToF64 + humansize::Unsigned>(size: T) -> String {
    let res: String = format_size(size, DECIMAL);

    res
}

pub fn format_lensize(size: u64) -> String {
    let sizes = ["", "k", "M", "G", "T", "P", "E", "Z"];
    let mut size: f64 = size as f64;

    let mut count = 0;
    while count < sizes.len() - 1 && size > 1000.0 {
        size /= 1000.0;
        count += 1;
    }

    format!("{:.2}{}", size, sizes[count])
}

pub fn is_image(extension: String) -> bool {
    extension.eq("png")
        || extension.eq("jpg")
        || extension.eq("jpeg")
        || extension.eq("ico")
        || extension.eq("gif")
        || extension.eq("bmp")
        || extension.eq("svg")
}

