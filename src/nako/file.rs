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
    pub fn exists(&self, f: &str) -> bool {
        if path::Path::new(f).exists() {
            return true;
        }

        false
    }

    /// 删除文件
    pub fn remove(&self, f: &str) -> Result<(), Error> {
        fs::remove_file(f)?;

        Ok(())
    }

    /// 创建文件
    pub fn create(&self, f: &str) -> Result<fs::File, Error> {
        let file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(f)?;

        Ok(file)
    }

    /// 读取
    pub fn read(&self, f: &str) -> Result<String, Error> {
        let mut file = fs::File::open(f)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let contents = String::from_utf8_lossy(&buffer).to_string();

        Ok(contents)
    }

    /// 写入信息
    pub fn write(&self, f: &str, content: String) -> Result<(), Error> {
        let path = path::Path::new(f);

        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// 创建文件夹
    pub fn mkdir(&self, d: &str) -> Result<(), Error> {
        fs::create_dir_all(d)?;

        Ok(())
    }

    /// 删除文件夹
    pub fn rmdir(&self, d: &str) -> Result<(), Error> {
        fs::remove_dir(d)?;

        Ok(())
    }

}