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
    let view = &state.view;

    let mut ctx = tera::Context::new();
    ctx.insert("name", "hello");

    Ok(nako_http::view(view, "admin/user/index.html", &ctx))
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
    name: Option<String>,
}

// 数据列表
pub async fn list(
    state: web::Data<AppState>,
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    let page: u64 = query.page;
    let per_page: u64 = query.limit;

    let name: String = match query.name.clone() {
        Some(res) => res,
        None => "".to_string(),
    };

    let mut res: ListData = ListData{
        list: vec![],
        count: 0,
    };

    if name.as_str() != "" {
        let keyword = "%".to_owned() + name.as_str() + "%";

        let data = user::UserModel::find_users_in_page_by_name(db, page, per_page, keyword.as_str()).await;
        let (user_list, _num_pages) = match data { 
            Ok((user_list, num_pages)) => (user_list, num_pages),
            Err(_) => {
                let list: Vec<user_entity::Model> = Vec::new();
    
                (list, 0)
            },
        };
    
        let count = match user::UserModel::find_users_count_by_name(db, keyword.as_str()).await {
            Ok(res) => res,
            Err(_) => 0,
        };

        res = ListData{
            list: user_list,
            count: count,
        };
    } else {
        let data = user::UserModel::find_users_in_page(db, page, per_page).await;
        let (user_list, _num_pages) = match data { 
            Ok((user_list, num_pages)) => (user_list, num_pages),
            Err(_) => {
                let list: Vec<user_entity::Model> = Vec::new();
    
                (list, 0)
            },
        };
    
        let count = match user::UserModel::find_users_count(db).await {
            Ok(res) => res,
            Err(_) => 0,
        };

        res = ListData{
            list: user_list,
            count: count,
        };
    }

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
    let view = &state.view;

    if query.id == 0 {
        return Ok(nako_http::error_response_html(&view, "ID不能为空", ""));
    }

    let user_data = user::UserModel::find_user_by_id(db, query.id).await;
    if let Err(_) = user_data {
        return Ok(nako_http::error_response_html(&view, "账号不存在", ""));
    }

    let mut ctx = tera::Context::new();
    
    if let Ok(Some(user_info)) = user_data {
        ctx.insert("data", &user_info);
    }

    Ok(nako_http::view(view, "admin/user/detail.html", &ctx))
}

// ==========================

// 添加
pub async fn create(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let view = &state.view;

    let ctx = tera::Context::new();

    Ok(nako_http::view(view, "admin/user/create.html", &ctx))
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

    let user_data = user::UserModel::find_user_by_name(db, params.username.as_str()).await;
    if let Ok(Some(_user_info)) = user_data {
        return Ok(nako_http::error_response_json("账号已经存在"));
    }

    let mut ip: String = "0.0.0.0".to_string();
    if let Some(val) = req.peer_addr() {
        ip = val.ip().to_string();
    }

    let add_time = time::now().timestamp();

    let create_data = user::UserModel::create_user(db, user_entity::Model{
            username: params.username.clone(),
            nickname: params.nickname.clone(),
            sign:     Some(params.sign.clone()),
            status:   Some(params.status),
            add_time: Some(add_time),
            add_ip:   Some(ip.clone()),
            ..entity::default()
        }).await;
    if let Ok(_create_info) = create_data {
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
    let view = &state.view;

    if query.id == 0 {
        return Ok(nako_http::error_response_html(&view, "ID不能为空", ""));
    }

    let user_data = user::UserModel::find_user_by_id(db, query.id).await;
    if let Err(_) = user_data {
        return Ok(nako_http::error_response_html(&view, "账号不存在", ""));
    }

    let mut ctx = tera::Context::new();
    
    if let Ok(Some(user_info)) = user_data {
        ctx.insert("data", &user_info);
    }

    Ok(nako_http::view(view, "admin/user/update.html", &ctx))
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

    let user_data = user::UserModel::find_user_by_id(db, query.id).await;
    if let Err(_) = user_data {
        return Ok(nako_http::error_response_json("要更改的账号不存在"));
    }

    let user_data_by_name = user::UserModel::find_user_by_name(db, params.username.as_str()).await;
    if let Ok(Some(user_info2)) = user_data_by_name {
        if let Ok(Some(user_info1)) = user_data {
            if user_info2.id != user_info1.id {
                return Ok(nako_http::error_response_json("账号已经存在"));
            }
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
    if let Err(_) = user_data {
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
    if query.id == 0 {
        return Ok(nako_http::error_response_json("ID不能为空"));
    }

    let admin_id: u32 = app::get_admin_id();
    if admin_id == query.id {
        return Ok(nako_http::error_response_json("当前账号不能被删除"));
    }

    if let Some(login_id) = session.get::<u32>("login_id")? {
        if login_id == query.id {
            return Ok(nako_http::error_response_json("你不能删除你自己"));
        }
    } 

    let db = &state.db;

    let user_data = user::UserModel::find_user_by_id(db, query.id).await;
    if let Err(_) = user_data {
        return Ok(nako_http::error_response_json("要删除的账号不存在"));
    }

    let delete_data = user::UserModel::delete_user(db, query.id).await;
    if let Err(_) = delete_data {
        return Ok(nako_http::error_response_json("删除失败"));
    }

    Ok(nako_http::success_response_json("删除成功", ""))
}

#[derive(Deserialize)]
pub struct UpdateStatusQuery {
    id: u32,
}

// ==========================

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

    let user_data = user::UserModel::find_user_by_id(db, query.id).await;
    if let Err(_) = user_data {
        return Ok(nako_http::error_response_json("要更改的账号不存在"));
    }

    // 更新
    let user_data = user::UserModel::update_status_by_id(db, query.id, user_entity::Model{
            status: Some(params.status),
            ..entity::default()
        })
        .await;
    if let Err(_) = user_data {
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
    let view = &state.view;

    if query.id == 0 {
        return Ok(nako_http::error_response_html(&view, "ID不能为空", ""));
    }

    let user_data = user::UserModel::find_user_by_id(db, query.id).await;
    if let Err(_) = user_data {
        return Ok(nako_http::error_response_html(&view, "账号不存在", ""));
    }

    let mut ctx = tera::Context::new();
    
    if let Ok(Some(user_info)) = user_data {
        ctx.insert("data", &user_info);
    }

    Ok(nako_http::view(view, "admin/user/update_passwqord.html", &ctx))
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

    let user_data = user::UserModel::find_user_by_id(db, query.id).await;
    if let Err(_) = user_data {
        return Ok(nako_http::error_response_json("要更改的账号不存在"));
    }

    let new_password = nako_auth::password_hash(params.password.as_str());
    if new_password.as_str() == "" {
        return Ok(nako_http::error_response_json("更改密码失败"));
    }

    // 更新
    let user_data = user::UserModel::update_password_by_id(db, query.id, user_entity::Model{
            password: Some(new_password.clone()),
            ..entity::default()
        })
        .await;
    if let Err(_) = user_data {
        return Ok(nako_http::error_response_json("更新失败"));
    }

    Ok(nako_http::success_response_json("更新成功", ""))
}
