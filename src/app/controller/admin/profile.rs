use actix_web::{
    web, 
    Result, 
    Error, 
    HttpResponse, 
};

use crate::nako::{
    auth as nako_auth,
    http as nako_http,
};
use crate::nako::global::{
    Session, 
    AppState,
    Deserialize,
};

use crate::app::service::http;
use crate::app::entity::{
    self,
    user as user_entity
};
use crate::app::model::{
    user,
};

// 更新信息
pub async fn update_info(
    state: web::Data<AppState>,
    session: Session, 
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    let id = session.get::<u32>("login_id").unwrap_or_default().unwrap_or_default();

    let user_info = user::UserModel::find_user_by_id(db, id).await.unwrap_or_default().unwrap_or_default();
    if user_info.id == 0 {
        return Ok(http::error_admin_html(&mut view, "账号不存在", ""));
    }

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &user_info);

    Ok(nako_http::view(&mut view, "admin/profile/update_info.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdateForm {
    username: String,
    nickname: String,
    sign: String,
}

// 更新信息保存
pub async fn update_info_save(
    state: web::Data<AppState>,
    params: web::Form<UpdateForm>,
    session: Session, 
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if params.username.as_str() == "" {
        return Ok(nako_http::error_json("账号不能为空"));
    }
    if params.nickname.as_str() == "" {
        return Ok(nako_http::error_json("昵称不能为空"));
    }

    let id = session.get::<u32>("login_id").unwrap_or_default().unwrap_or_default();

    let user_info = user::UserModel::find_user_by_id(db, id).await.unwrap_or_default().unwrap_or_default();
    if user_info.id == 0 {
        return Ok(nako_http::error_json("账号不存在"));
    }

    let user_info_by_name = user::UserModel::find_user_by_name(db, params.username.as_str()).await.unwrap_or_default().unwrap_or_default();
    if user_info_by_name.id > 0 {
        if user_info_by_name.id != user_info.id {
            return Ok(nako_http::error_json("账号已经存在"));
        }
    }

    // 更新
    let user_data = user::UserModel::update_user_by_id(db, id, user_entity::Model{
            username: params.username.clone(),
            nickname: params.nickname.clone(),
            sign:     Some(params.sign.clone()),
            status:   Some(1),
            ..entity::default()
        })
        .await;
    if user_data.is_err() {
        return Ok(nako_http::error_json("更新失败"));
    }

    Ok(nako_http::success_json("更新成功", ""))
}

// ==========================

// 更改密码
pub async fn update_password(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/profile/update_password.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdatePasswordForm {
    old_password: String,
    new_password: String,
    new_password2: String,
}

// 更改密码保存
pub async fn update_password_save(
    state: web::Data<AppState>,
    params: web::Form<UpdatePasswordForm>,
    session: Session, 
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if params.old_password.as_str() == "" {
        return Ok(nako_http::error_json("旧密码不能为空"));
    }
    if params.new_password.as_str() == "" {
        return Ok(nako_http::error_json("新密码不能为空"));
    }
    if params.new_password2.as_str() == "" {
        return Ok(nako_http::error_json("确认密码不能为空"));
    }
    if params.new_password2 != params.new_password {
        return Ok(nako_http::error_json("确认密码不一致"));
    }

    let mut id: u32 = 0;
    if let Some(login_id) = session.get::<u32>("login_id")? {
        id = login_id;
    } 

    let user_info = user::UserModel::find_user_by_id(db, id).await.unwrap_or_default().unwrap_or_default();
    if user_info.id == 0 {
        return Ok(nako_http::error_json("更改密码失败"));
    }

    let pass = user_info.password.unwrap_or("".to_string());

    if !nako_auth::password_verify(params.old_password.as_str(), pass.as_str()) {
        return Ok(nako_http::error_json("账号旧密码错误"));
    }

    let new_hashed_password = nako_auth::password_hash(params.new_password.as_str());
    if new_hashed_password.as_str() == "" {
        return Ok(nako_http::error_json("更改密码失败"));
    }

    // 更新
    let new_user_data = user::UserModel::update_password_by_id(db, id, user_entity::Model{
            password: Some(new_hashed_password.clone()),
            ..entity::default()
        })
        .await;
    if new_user_data.is_err() {
        return Ok(nako_http::error_json("更改密码失败"));
    }

    Ok(nako_http::success_json("更新密码成功", ""))
}

// ==========================

// 更改头像
pub async fn update_avatar(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/profile/update_avatar.html", &ctx))
}

// 更改头像数据
#[derive(Deserialize)]
pub struct UpdateAvatarForm {
    avatar: String,
}

// 更改头像保存
pub async fn update_avatar_save(
    state: web::Data<AppState>,
    params: web::Form<UpdateAvatarForm>,
    session: Session, 
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if params.avatar.as_str() == "" {
        return Ok(nako_http::error_json("头像数据不能为空"));
    }

    let mut id: u32 = 0;
    if let Some(login_id) = session.get::<u32>("login_id")? {
        id = login_id;
    } 

    let user_info = user::UserModel::find_user_by_id(db, id).await.unwrap_or_default().unwrap_or_default();
    if user_info.id == 0 {
        return Ok(nako_http::error_json("更改头像失败"));
    }

    // 更新
    let new_user_data = user::UserModel::update_avatar_by_id(db, id, user_entity::Model{
            avatar: Some(params.avatar.clone()),
            ..entity::default()
        })
        .await;
    if new_user_data.is_err() {
        return Ok(nako_http::error_json("更改头像失败"));
    }

    Ok(nako_http::success_json("更改头像成功", ""))
}
