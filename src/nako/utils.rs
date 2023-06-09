use std::path::Path;
use std::ffi::OsStr;

use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::digest::Digest;
use uuid::Uuid;
use humansize::{format_size, DECIMAL};
use data_encoding::BASE64;

use actix_web::HttpRequest;

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

// base64 编码
pub fn base64_encode(data: &[u8]) -> String {
    BASE64.encode(data)
}

// base64 解码
pub fn base64_decode(data: String) -> Vec<u8> {
    let data = data.as_bytes();

    let res = BASE64.decode(data).unwrap_or_default();

    res
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

/// 删除空格
pub fn str_trim(input: &str) -> String {
    input
        // 修剪前面和后面的空格
        .trim()
        // 分割成行
        .lines()
        .map(|part| {
            // 对于每一行
            part
                // 修剪前导和尾部的空白处
                .trim()
                //对空白处进行分割。
                //包括字符串被分割的空白处
                // 分割后的部分
                .split_inclusive(char::is_whitespace)
                // 过滤掉只包含空白的子字符串
                .filter(|part| !part.trim().is_empty())
                //为这一行收集成一个字符串
                .collect()
        })
        //收集成一个字符串的Vec
        .collect::<Vec<String>>()
        //用换行符连接这些字符串
        //返回到最终的字符串中
        .join("")
}

/// 生成 url
pub fn url_for<U, I>(req: HttpRequest, name: &str, elements: U) -> String 
where
    U: IntoIterator<Item = I>,
    I: AsRef<str>,
{
    let url: String = match req.url_for(name, elements) {
        Ok(data) => data.into(),
        Err(_) => "/".into(),
    };

    url
}

/// 生成 url 不带参数
pub fn url_for_static(req: HttpRequest, name: &str) -> String {
    let url: String = match req.url_for_static(name) {
        Ok(data) => data.into(),
        Err(_) => "/".into(),
    };

    url
}

