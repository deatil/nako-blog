use actix_web::{
    web, 
    Result, 
    Error, 
    HttpRequest,
    HttpResponse, 
};

use crate::nako::{
    app,
    time,
    auth as nako_auth,
    http as nako_http,
};
use crate::nako::global::{
    Session, 
    AppState,
    Serialize,
    Deserialize,
};

use crate::app::entity::{
    self,
    user as user_entity
};
use crate::app::model::{
    user,
};

// 首页
pub async fn index(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/user/index.html", &ctx))
}

// ==========================

#[derive(Serialize)]
pub struct ListData {
    list: Vec<user_entity::Model>,
    count: u64,
}

#[derive(Deserialize)]
pub struct ListQuery {
    page: u64,
    limit: u64,

    username: Option<String>,
    nickname: Option<String>,
    status: Option<i32>,
}

// 数据列表
pub async fn list(
    state: web::Data<AppState>,
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    let page: u64 = query.page;
    let per_page: u64 = query.limit;

    let search_where = user::UserWhere{
        username: query.username.clone(),
        nickname: query.nickname.clone(),
        status: query.status,
    };
    let search_where = search_where.format();

    let (list, _num_pages) = user::UserModel::search_in_page(
            db, 
            page, 
            per_page, 
            search_where.clone(),
        )
        .await.unwrap_or_default();
    let count = user::UserModel::search_count(db, search_where.clone())
        .await.unwrap_or(0);

    let res: ListData = ListData{
        list: list,
        count: count,
    };

    Ok(nako_http::success_response_json("获取成功", res))
}

// ==========================

#[derive(Deserialize)]
pub struct DetailQuery {
    id: u32,
}

// 详情
pub async fn detail(
    state: web::Data<AppState>,
    query: web::Query<DetailQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    if query.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "ID不能为空", ""));
    }

    let user_data = user::UserModel::find_user_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if user_data.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "账号不存在", ""));
    }

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &user_data);

    Ok(nako_http::view(&mut view, "admin/user/detail.html", &ctx))
}

// ==========================

// 添加
pub async fn create(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/user/create.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct CreateForm {
    username: String,
    nickname: String,
    sign: String,
    status: i32,
}

// 添加保存
pub async fn create_save(
    req: HttpRequest,
    state: web::Data<AppState>,
    params: web::Form<CreateForm>,
) -> Result<HttpResponse, Error> {
    if params.username.as_str() == "" {
        return Ok(nako_http::error_response_json("账号不能为空"));
    }
    if params.nickname.as_str() == "" {
        return Ok(nako_http::error_response_json("昵称不能为空"));
    }
    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_response_json("状态不能为空"));
    }

    let db = &state.db;

    let user_data = user::UserModel::find_user_by_name(db, params.username.as_str()).await.unwrap_or_default().unwrap_or_default();
    if user_data.id > 0 {
        return Ok(nako_http::error_response_json("账号已经存在"));
    }

    let add_time = time::now().timestamp();

    let mut ip: String = "0.0.0.0".to_string();
    if let Some(val) = req.peer_addr() {
        ip = val.ip().to_string();
    }

    let create_data = user::UserModel::create_user(db, user_entity::Model{
            username: params.username.clone(),
            nickname: params.nickname.clone(),
            sign:     Some(params.sign.clone()),
            status:   Some(params.status),
            add_time: Some(add_time),
            add_ip:   Some(ip.clone()),
            ..entity::default()
        }).await;
    if let Ok(_) = create_data {
        return Ok(nako_http::success_response_json("添加成功", ""));
    }

    Ok(nako_http::error_response_json("添加失败"))
}

// ==========================

#[derive(Deserialize)]
pub struct UpdateQuery {
    id: u32,
}

// 更新
pub async fn update(
    state: web::Data<AppState>,
    query: web::Query<UpdateQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    if query.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "ID不能为空", ""));
    }

    let user_info = user::UserModel::find_user_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if user_info.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "账号不存在", ""));
    }

    let mut ctx = nako_http::view_data();
    
    ctx.insert("data", &user_info);

    Ok(nako_http::view(&mut view, "admin/user/update.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdateForm {
    username: String,
    nickname: String,
    sign: String,
    status: i32,
}

// 更新保存
pub async fn update_save(
    state: web::Data<AppState>,
    query: web::Query<UpdateQuery>,
    params: web::Form<UpdateForm>,
    session: Session, 
) -> Result<HttpResponse, Error> {
    if query.id == 0 {
        return Ok(nako_http::error_response_json("ID不能为空"));
    }

    if params.username.as_str() == "" {
        return Ok(nako_http::error_response_json("账号不能为空"));
    }
    if params.nickname.as_str() == "" {
        return Ok(nako_http::error_response_json("昵称不能为空"));
    }
    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_response_json("状态不能为空"));
    }

    let admin_id: u32 = app::get_admin_id();
    if admin_id == query.id {
        return Ok(nako_http::error_response_json("当前账号不能被修改"));
    }

    if let Some(login_id) = session.get::<u32>("login_id")? {
        if login_id == query.id {
            return Ok(nako_http::error_response_json("你不能修改你自己"));
        }
    } 

    let db = &state.db;

    let user_info = user::UserModel::find_user_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if user_info.id == 0 {
        return Ok(nako_http::error_response_json("要更改的账号不存在"));
    }

    let user_info_by_name = user::UserModel::find_user_by_name(db, params.username.as_str()).await.unwrap_or_default().unwrap_or_default();
    if user_info_by_name.id > 0 {
        if user_info.id != user_info_by_name.id {
            return Ok(nako_http::error_response_json("账号已经存在"));
        }
    }

    // 更新
    let user_data = user::UserModel::update_user_by_id(db, query.id, user_entity::Model{
            username: params.username.clone(),
            nickname: params.nickname.clone(),
            sign:     Some(params.sign.clone()),
            status:   Some(params.status),
            ..entity::default()
        })
        .await;
    if user_data.is_err() {
        return Ok(nako_http::error_response_json("更新失败"));
    }

    Ok(nako_http::success_response_json("更新成功", ""))
}

// ==========================

#[derive(Deserialize)]
pub struct DeleteQuery {
    id: u32,
}

// 删除
pub async fn delete(
    state: web::Data<AppState>,
    query: web::Query<DeleteQuery>,
    session: Session, 
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if query.id == 0 {
        return Ok(nako_http::error_response_json("ID不能为空"));
    }

    let admin_id: u32 = app::get_admin_id();
    if admin_id == query.id {
        return Ok(nako_http::error_response_json("当前账号不能被删除"));
    }

    let login_id = session.get::<u32>("login_id").unwrap_or_default().unwrap_or_default();
    if login_id == query.id {
        return Ok(nako_http::error_response_json("你不能删除你自己"));
    }

    let user_data = user::UserModel::find_user_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if user_data.id == 0 {
        return Ok(nako_http::error_response_json("要删除的账号不存在"));
    }

    let delete_data = user::UserModel::delete_user(db, query.id).await;
    if delete_data.is_err() {
        return Ok(nako_http::error_response_json("删除失败"));
    }

    Ok(nako_http::success_response_json("删除成功", ""))
}

// ==========================

#[derive(Deserialize)]
pub struct UpdateStatusQuery {
    id: u32,
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdateStatusForm {
    status: i32,
}

// 更新保存
pub async fn update_status(
    state: web::Data<AppState>,
    query: web::Query<UpdateStatusQuery>,
    params: web::Form<UpdateStatusForm>,
    session: Session, 
) -> Result<HttpResponse, Error> {
    if query.id == 0 {
        return Ok(nako_http::error_response_json("ID不能为空"));
    }

    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_response_json("状态不能为空"));
    }

    let admin_id: u32 = app::get_admin_id();
    if admin_id == query.id {
        return Ok(nako_http::error_response_json("当前账号不能被修改"));
    }

    if let Some(login_id) = session.get::<u32>("login_id")? {
        if login_id == query.id {
            return Ok(nako_http::error_response_json("你不能修改你自己"));
        }
    } 

    let db = &state.db;

    let user_data = user::UserModel::find_user_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if user_data.id == 0 {
        return Ok(nako_http::error_response_json("要更改的账号不存在"));
    }

    // 更新
    let user_data = user::UserModel::update_status_by_id(db, query.id, user_entity::Model{
            status: Some(params.status),
            ..entity::default()
        })
        .await;
    if user_data.is_err() {
        return Ok(nako_http::error_response_json("更新失败"));
    }

    Ok(nako_http::success_response_json("更新成功", ""))
}

// ==========================

#[derive(Deserialize)]
pub struct UpdatePasswordQuery {
    id: u32,
}

// 更改密码
pub async fn update_password(
    state: web::Data<AppState>,
    query: web::Query<UpdatePasswordQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    if query.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "ID不能为空", ""));
    }

    let user_info = user::UserModel::find_user_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if user_info.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "账号不存在", ""));
    }

    let mut ctx = nako_http::view_data();
    
    ctx.insert("data", &user_info);

    Ok(nako_http::view(&mut view, "admin/user/update_password.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdatePasswordForm {
    password: String,
}

// 更改密码保存
pub async fn update_password_save(
    state: web::Data<AppState>,
    query: web::Query<UpdatePasswordQuery>,
    params: web::Form<UpdatePasswordForm>,
    session: Session, 
) -> Result<HttpResponse, Error> {
    if query.id == 0 {
        return Ok(nako_http::error_response_json("ID不能为空"));
    }

    if params.password.as_str() == "" {
        return Ok(nako_http::error_response_json("密码不能为空"));
    }

    let admin_id: u32 = app::get_admin_id();
    if admin_id == query.id {
        return Ok(nako_http::error_response_json("当前账号不能被修改"));
    }

    if let Some(login_id) = session.get::<u32>("login_id")? {
        if login_id == query.id {
            return Ok(nako_http::error_response_json("你不能修改你自己"));
        }
    } 

    let db = &state.db;

    let user_info = user::UserModel::find_user_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if user_info.id == 0 {
        return Ok(nako_http::error_response_json("要更改的账号不存在"));
    }

    let new_password = nako_auth::password_hash(params.password.as_str());
    if new_password.as_str() == "" {
        return Ok(nako_http::error_response_json("更改密码失败"));
    }

    // 更新
    let new_user_data = user::UserModel::update_password_by_id(db, query.id, user_entity::Model{
            password: Some(new_password.clone()),
            ..entity::default()
        })
        .await;
    if new_user_data.is_err() {
        return Ok(nako_http::error_response_json("更新失败"));
    }

    Ok(nako_http::success_response_json("更新成功", ""))
}
