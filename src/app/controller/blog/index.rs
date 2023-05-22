use actix_web::{web, error, Result, Error, Responder};
use actix_web_lab::respond::Html;
use actix_web::web::Json;

use crate::nako::utils;
use crate::nako::global::{
    Session, AppState,
    Status, ResponseEntity
};
use crate::app::entity::{
    self,
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

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        counter = count + 1;
        session.insert("counter", counter)?;
    } else {
        session.insert("counter", counter)?;
    }

    let mut ctx = tera::Context::new();
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

    /*
    // 创建
    user::UserModel::create_user(db, user_entity::Model{
            username: String::from("username"),
            nickname: String::from("nickname"),
            ..entity::default()
        })
        .await
        .expect("could not insert user");
    */

    /*
    // 查询
    let id: u32 = 1;
    let user_info: user_entity::Model = user::UserModel::find_user_by_id(db, id)
        .await
        .expect("could not find user")
        .unwrap_or_else(|| panic!("could not find user with id {id}"));
    */

    // 分页
    let page: u64 = 1;
    let per_page: u64 = 10;
    let (user_list, _num_pages) = user::UserModel::find_users_in_page(db, page, per_page)
        .await
        .expect("Cannot find users in page");

    // 更新
    let id: u32 = 1;
    user::UserModel::update_user_by_id(db, id, user_entity::Model{
            username: String::from("username123"),
            nickname: String::from("nickname123"),
            sign: Option::Some(String::from("signsign")),
            ..entity::default()
        })
        .await
        .expect("could not edit user");

    /*
    // 删除
    let id2: u32 = 2;
    user::UserModel::delete_user(db, id2)
        .await
        .expect("could not delete user");
    */

    /*
    // 清空表
    user::UserModel::delete_all_users(db)
        .await
        .expect("could not delete all user");
    */

    let data: ResponseEntity<Vec<user_entity::Model>> = ResponseEntity {
        status: Status::SUCCESS,
        code: 200,
        message: "获取成功".to_string(),
        data: Some(user_list),
    };

    Ok(Json(data))
}
