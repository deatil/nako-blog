use std::collections::HashMap;
use actix_web::{
    web, 
    Error, 
    Result, 
    HttpResponse,
};

use crate::nako::{
    app,
    http as nako_http,
};
use crate::nako::global::{
    AppState,
};

use crate::app::model::{
    art,
    cate,
    tag,
};

/// 根据名称查询分类
pub async fn index(
    state: web::Data<AppState>,
    name: web::Path<String>,
    web::Query(params): web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    let mut page: u64 = 1;
    if let Some(p) = params.get("page") {
        page = p.parse::<u64>().unwrap_or(1);
    }

    let per_page = 6;

    let search_where = art::ArtWhere{
        title: None,
        uuid: None,
        tag: Some(name.clone()),
        cate_id: None,
        user_id: None,
        is_top: None,
        status: Some(1),
    };
    let (arts, num_pages) = art::ArtModel::search_in_page(db, page, per_page, search_where.clone()).await.unwrap_or_default();
    
    // 标签信息
    let tag_data = tag::TagModel::find_by_name(db, name.clone().as_str()).await.unwrap_or_default().unwrap_or_default();

    let hot_arts = art::ArtModel::find_one_year_hot(db, 6).await.unwrap_or_default();
    let cates = cate::CateModel::find_open_cate(db).await.unwrap_or_default();
    let tags = tag::TagModel::find_open_tags(db, 6).await.unwrap_or_default();

    let mut ctx = nako_http::view_data();
    ctx.insert("arts", &arts);
    ctx.insert("page", &page);
    ctx.insert("num_pages", &num_pages);
   
    ctx.insert("tag_name", &name.clone());
    ctx.insert("tag_data", &tag_data);

    ctx.insert("hot_arts", &hot_arts);
    ctx.insert("cates", &cates);
    ctx.insert("tags", &tags);

    Ok(nako_http::view(&mut view, app::view_path("tag.html").as_str(), &ctx))
}


