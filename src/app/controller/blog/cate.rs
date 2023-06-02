use std::collections::HashMap;
use actix_web::{
    web, 
    Error, 
    Result, 
    HttpRequest,
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

/// 分类全部列表
pub async fn index(
    state: web::Data<AppState>,
    req: HttpRequest,
    web::Query(params): web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let view = &state.view;

    let mut page: u64 = 1;
    if let Some(p) = params.get("page") {
        page = p.parse::<u64>().unwrap_or(1);
    }

    let mut keywords: String = "".to_string();
    if let Some(k) = params.get("keywords") {
        keywords = k.parse::<String>().unwrap_or("".into());
    }

    let mut title = None;
    if keywords.as_str() != "" {
        title = Some(keywords.clone());
    }

    let per_page = 6;

    let search_where = art::ArtWhere{
        uuid: None,
        title: title,
        tag: None,
        cate_id: None,
        user_id: None,
        is_top: None,
        status: Some(1),
    };
    let (arts, num_pages) = art::ArtModel::search_in_page(db, page, per_page, search_where.clone()).await.unwrap_or_default();
    let _count = art::ArtModel::search_count(db, search_where.clone()).await.unwrap_or(0);

    let _path_query = match req.uri().path_and_query() {
        Some(path_query) => path_query.as_str(),
        _ => "",
    };

    let hot_arts = art::ArtModel::find_one_year_hot(db, 6).await.unwrap_or_default();
    let cates = cate::CateModel::find_open_cate(db).await.unwrap_or_default();
    let tags = tag::TagModel::find_open_tags(db, 6).await.unwrap_or_default();

    let mut ctx = nako_http::view_data();
    ctx.insert("arts", &arts);
    ctx.insert("page", &page);
    ctx.insert("num_pages", &num_pages);
 
    ctx.insert("keywords", &keywords);

    ctx.insert("hot_arts", &hot_arts);
    ctx.insert("cates", &cates);
    ctx.insert("tags", &tags);

    Ok(nako_http::view(view, app::view_path("list.html").as_str(), &ctx))
}

/// 根据名称查询分类
pub async fn name(
    state: web::Data<AppState>,
    slug: web::Path<String>,
    web::Query(params): web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let view = &state.view;

    let mut page: u64 = 1;
    if let Some(p) = params.get("page") {
        page = p.parse::<u64>().unwrap_or(1);
    }

    let per_page = 6;

    // 分类
    let cate_data = cate::CateModel::find_by_slug(db, slug.as_str())
        .await.unwrap_or_default().unwrap_or_default();
    if cate_data.id == 0 {
        return Ok(app::error_html(view, "分类不存在"));
    }

    let search_where = art::ArtWhere{
        title: None,
        uuid: None,
        tag: None,
        cate_id: Some(cate_data.id),
        user_id: None,
        is_top: None,
        status: Some(1),
    };
    let (arts, num_pages) = art::ArtModel::search_in_page(db, page, per_page, search_where.clone()).await.unwrap_or_default();

    let hot_arts = art::ArtModel::find_one_year_hot(db, 6).await.unwrap_or_default();
    let cates = cate::CateModel::find_open_cate(db).await.unwrap_or_default();
    let tags = tag::TagModel::find_open_tags(db, 6).await.unwrap_or_default();

    let mut ctx = nako_http::view_data();
    ctx.insert("arts", &arts);
    ctx.insert("page", &page);
    ctx.insert("num_pages", &num_pages);
   
    ctx.insert("cate_data", &cate_data);

    ctx.insert("hot_arts", &hot_arts);
    ctx.insert("cates", &cates);
    ctx.insert("tags", &tags);

    Ok(nako_http::view(view, app::view_path(cate_data.list_tpl.as_str()).as_str(), &ctx))
}


