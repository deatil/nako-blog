use std::{
    fs, 
    path,
    io::Read,
};

use actix_web::{
    web, 
    Result, 
    Error, 
    HttpRequest,
    HttpResponse, 
};
use actix_multipart::{
    form::{
        tempfile::{
            TempFile, 
        },
        MultipartForm,
    },
};
use sea_orm::TryIntoModel;

use crate::nako::{
    time,
    utils,
    http as nako_http,
    app::{
        attach_path,
        upload_path, 
        upload_url,    
    }
};
use crate::nako::global::{
    Session, 
    AppState,
    Serialize,
};

use crate::app::entity::{
    self,
    attach as attach_entity
};
use crate::app::model::{
    attach,
};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[derive(Serialize)]
pub struct FileData {
    id: u32,
}

// 上传文件
pub async fn file(
    req: HttpRequest,
    state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    let dir = upload_path("".to_string());

    if !path::Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.as_str())?;
    }

    let add_time = time::now().timestamp();

    let add_ip: String = match req.peer_addr() {
        Some(val) => val.ip().to_string(),
        None => "0.0.0.0".to_string(),
    };

    let mut res = Vec::new();

    for mut f in form.files {
        let file_name = f.file_name.unwrap_or("".to_string());
        if file_name.as_str() == "" {
            continue;
        }

        let ext = utils::get_extension(file_name.clone().as_str());
        let name = format!("/{}.{}", utils::uuid(), ext);

        let path = attach_path(name.clone());

        let mut buffer = Vec::new();
        if f.file.read_to_end(&mut buffer).is_err() {
            return Ok(nako_http::error_json("上传失败"));
        }

        let contents = String::from_utf8_lossy(&buffer).to_string();

        let md5 = utils::md5(contents.as_str());
        let size = buffer.len() as u64;

        // 判断是否有相同
        let attach_data = attach::AttachModel::find_by_md5(db, md5.as_str()).await.unwrap_or_default().unwrap_or_default();
        if attach_data.id > 0 {
            res.push(FileData{
                id: attach_data.id,
            });

            continue;
        }

        if f.file.persist(path.clone()).is_err() {
            return Ok(nako_http::error_json("上传失败"));
        }

        let create_data = attach::AttachModel::create(db, attach_entity::Model{
                name:     file_name.clone(),
                path:     name.clone(),
                ext:      ext.clone(),
                size:     size,
                md5:      md5.clone(),
                r#type:   i32::from(1),
                status:   i32::from(1),
                add_time: add_time,
                add_ip:   add_ip.clone(),
                ..entity::default()
            }).await;
        if let Ok(data) = create_data {
            if let Ok(data_model) = data.try_into_model() {
                res.push(FileData{
                    id:  data_model.id,
                });
            }
        } else {
            if let Ok(_) = fs::remove_file(path.clone()) {}

            return Ok(nako_http::error_json("上传失败"));
        }
    }

    Ok(nako_http::success_json("上传成功", res))
}

// =================

#[derive(Debug, MultipartForm)]
pub struct ImageForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[derive(Serialize)]
pub struct ImageData {
    id: u32,
    url: String,
}

// 上传图片
pub async fn image(
    req: HttpRequest,
    state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<ImageForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    let dir = upload_path("".to_string());

    if !path::Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.as_str())?;
    }

    let add_time = time::now().timestamp();

    let add_ip: String = match req.peer_addr() {
        Some(val) => val.ip().to_string(),
        None => "0.0.0.0".to_string(),
    };

    let mut res = Vec::new();

    for mut f in form.files {
        let file_name = f.file_name.unwrap_or("".to_string());
        if file_name.as_str() == "" {
            continue;
        }

        let ext = utils::get_extension(file_name.clone().as_str());
        let name = format!("/images/{}.{}", utils::uuid(), ext);

        let path = upload_path(name.clone());
        let url = upload_url(name.clone());

        let mut buffer = Vec::new();
        if f.file.read_to_end(&mut buffer).is_err() {
            return Ok(nako_http::error_json("上传失败"));
        }

        let contents = String::from_utf8_lossy(&buffer).to_string();

        let md5 = utils::md5(contents.as_str());
        let size = buffer.len() as u64;

        // 判断是否有相同
        let attach_data = attach::AttachModel::find_by_md5(db, md5.as_str()).await.unwrap_or_default().unwrap_or_default();
        if attach_data.id > 0 {
            res.push(ImageData{
                id:  attach_data.id,
                url: upload_url(attach_data.path),
            });

            continue;
        }

        if f.file.persist(path.clone()).is_err() {
            return Ok(nako_http::error_json("上传失败"));
        }

        let create_data = attach::AttachModel::create(db, attach_entity::Model{
                name:     file_name.clone(),
                path:     name.clone(),
                ext:      ext.clone(),
                size:     size,
                md5:      md5.clone(),
                r#type:   i32::from(2),
                status:   i32::from(1),
                add_time: add_time,
                add_ip:   add_ip.clone(),
                ..entity::default()
            }).await;
        if let Ok(data) = create_data {
            if let Ok(data_model) = data.try_into_model() {
                res.push(ImageData{
                    id:  data_model.id,
                    url: url,
                });
            }
        } else {
            if let Ok(_) = fs::remove_file(path.clone()) {}

            return Ok(nako_http::error_json("上传失败"));
        }
    }

    Ok(nako_http::success_json("上传成功", res))
}

// =================

#[derive(Serialize)]
pub struct AvatarData {
    url: String,
}

#[derive(MultipartForm)]
pub struct AvatarForm {
    file: TempFile,
}

// 上传头像
pub async fn avatar(
    session: Session, 
    form: MultipartForm<AvatarForm>
) -> Result<HttpResponse, Error> {
    let form = form.into_inner();

    let file_name = match form.file.file_name {
        Some(v) => v,
        None => "".to_string(),
    };

    if file_name.as_str() == "" {
        return Ok(nako_http::error_json("上传失败"));
    }

    let avatar_dir = upload_path("avatar/".to_string());

    if !path::Path::new(avatar_dir.as_str()).exists() {
        fs::create_dir_all(avatar_dir.as_str())?;
    }

    let mut id: u32 = 0;
    if let Some(login_id) = session.get::<u32>("login_id")? {
        id = login_id;
    } 

    let name = utils::sha1(id.to_string().as_str());

    let ext = utils::get_extension(file_name.clone().as_str());
    let name = format!("/avatar/{}.{}", name.clone(), ext);
    let path = upload_path(name.clone());
    let url = upload_url(name.clone());

    if form.file.file.persist(path.clone()).is_err() {
        return Ok(nako_http::error_json("上传失败"));
    }

    let res = AvatarData{
        url: url,
    };
    
    Ok(nako_http::success_json("上传成功", res))
}
