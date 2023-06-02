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
    art,
    cate,
    comment,
    tag,
};

// 首页
pub async fn index(
    state: web::Data<AppState>,
    session: Session, 
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let view = &state.view;

    let id = session.get::<u32>("login_id").unwrap_or_default().unwrap_or_default();
    let user_info = user::UserModel::find_user_by_id(db, id).await.unwrap_or_default().unwrap_or_default();
    
    let mut ctx = nako_http::view_data();
    ctx.insert("login_user", &user_info);

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
    let db = &state.db;
    let view = &state.view;

    let search_where = art::ArtWhere{
        title: None,
        uuid: None,
        tag: None,
        cate_id: None,
        user_id: None,
        is_top: None,
        status: Some(1),
    };
    let (new_arts, _) = art::ArtModel::search_in_page(db, 1, 6, search_where.clone()).await.unwrap_or_default();

    let art_count = art::ArtModel::find_count(db).await.unwrap_or(0);
    let cate_count = cate::CateModel::find_count(db).await.unwrap_or(0);
    let comment_count = comment::CommentModel::find_count(db).await.unwrap_or(0);
    let tag_count = tag::TagModel::find_count(db).await.unwrap_or(0);

    let mut ctx = nako_http::view_data();
    ctx.insert("new_arts", &new_arts);

    ctx.insert("art_count", &art_count);
    ctx.insert("cate_count", &cate_count);
    ctx.insert("comment_count", &comment_count);
    ctx.insert("tag_count", &tag_count);

    Ok(nako_http::view(view, "admin/index/console.html", &ctx))
}
