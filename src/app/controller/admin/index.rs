use actix_web::{
    web, 
    Result, 
    Error, 
    HttpResponse, 
};

use crate::nako::http as nako_http;
use crate::nako::global::{
    Session, 
    AppState,
};

use crate::app::data as app_data;
use crate::app::model::{
    user,
};

// 首页
pub async fn index(
    state: web::Data<AppState>,
    session: Session, 
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let view = &state.view;

    let mut ctx = nako_http::view_data();

    if let Some(login_id) = session.get::<u32>("login_id")? {
        let user_data = user::UserModel::find_user_by_id(db, login_id).await;

        if let Ok(Some(user_info)) = user_data {
            ctx.insert("login_user", &user_info);
        }
    } 

    Ok(nako_http::view(view, "admin/index/index.html", &ctx))
}

// 菜单
pub async fn menu() -> Result<HttpResponse, Error> {
    let data = app_data::menu::menus();

    Ok(nako_http::json(data))
}

// 控制台
pub async fn console(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let view = &state.view;

    let mut ctx = nako_http::view_data();

    let art_count = 12;
    ctx.insert("art_count", &art_count);

    Ok(nako_http::view(view, "admin/index/console.html", &ctx))
}
