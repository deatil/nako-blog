use actix_web::{web, error, Result, Error, Responder};
use actix_web_lab::respond::Html;
use actix_web::web::Json;

use crate::nako::utils;
use crate::nako::http as nako_http;
use crate::nako::global::{
    Session, AppState,
    Status, ResponseEntity
};
use crate::app::entity::{
    user as user_entity
};
use crate::app::model::{
    user,
};

pub async fn index(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

pub async fn data(
    session: Session, 
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let view = &state.view;
    
    // let id: u32 = req.match_info().query("id").parse().unwrap_or_default();

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        counter = count + 1;
        session.insert("counter", counter)?;
    } else {
        session.insert("counter", counter)?;
    }

    let mut ctx = nako_http::view_data();
    ctx.insert("name", "hello");
    ctx.insert("text", "Welcome!");
    ctx.insert("counter", &counter);

    let s = view.render("blog/data.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(Html(s))
}

pub async fn json() -> Result<Json<ResponseEntity<String>>> {
    let s = utils::sha1("123112");

    let data: ResponseEntity<String> = ResponseEntity {
        status: Status::SUCCESS,
        code: 200,
        message: "获取成功".to_string(),
        data: Some(s),
    };

    Ok(Json(data))
}

pub async fn usedb(
    state: web::Data<AppState>,
) -> Result<Json<ResponseEntity<Vec<user_entity::Model>>>> {
    let db = &state.db;

    // 分页
    let page: u64 = 1;
    let per_page: u64 = 10;
    let (user_list, _num_pages) = user::UserModel::find_users_in_page(db, page, per_page)
        .await
        .expect("Cannot find users in page");

    let data: ResponseEntity<Vec<user_entity::Model>> = ResponseEntity {
        status: Status::SUCCESS,
        code: 200,
        message: "获取成功".to_string(),
        data: Some(user_list),
    };

    Ok(Json(data))
}
