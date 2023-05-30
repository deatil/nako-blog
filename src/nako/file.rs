use std::{
    fs, 
    path,
    io::{
        Read,
        Write,
        Error,
    },
};

/// 文件
pub struct File {}

impl File {
    pub fn new() -> Self {
        File{}
    }

    /// 判断是否存在
    pub fn exists(f: &str) -> bool {
        if path::Path::new(f).exists() {
            return true;
        }

        false
    }

    /// 删除文件
    pub fn remove(f: &str) -> Result<String, Error> {
        fs::remove_file(f)?;

        Ok(String::new())
    }

    /// 创建文件
    pub fn create(f: &str) -> Result<fs::File, Error> {
        let file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(f)?;

        Ok(file)
    }

    /// 读取
    pub fn read(f: &str) -> Result<String, Error> {
        let mut file = fs::File::open(f)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let contents = String::from_utf8_lossy(&buffer).to_string();

        Ok(contents)
    }

    /// 写入信息
    pub fn write(f: &str, content: String) -> Result<String, Error> {
        let path = path::Path::new(f);

        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(String::new())
    }

    /// 创建文件夹
    pub fn mkdir(d: &str) -> Result<String, Error> {
        fs::create_dir_all(d)?;

        Ok(String::new())
    }

    /// 删除文件夹
    pub fn rmdir(d: &str) -> Result<String, Error> {
        fs::remove_dir(d)?;

        Ok(String::new())
    }

}