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
    comment,
};

/// 详情
pub async fn index(
    state: web::Data<AppState>,
    uuid: web::Path<String>,
    web::Query(params): web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    let mut page: u64 = 1;
    if let Some(p) = params.get("page") {
        page = p.parse::<u64>().unwrap_or(1);
    }

    // 文章详情
    let art = art::ArtModel::find_by_uuid(db, uuid.as_str())
        .await.unwrap_or_default().unwrap_or_default();
    if art.id == 0 {
        return Ok(app::error_html(&mut view, "文章不存在"));
    }

    // 分类
    let cate_data = cate::CateModel::find_by_id(db, art.cate_id)
        .await.unwrap_or_default().unwrap_or_default();

    // 回复
    let (comments, comments_num_pages) = comment::CommentModel::find_in_page_by_artid(db, art.id, page, 6)
        .await.unwrap_or_default();
    let comments_count = comment::CommentModel::find_count_by_artid(db, art.id)
        .await.unwrap_or(0);

    let mut ctx = nako_http::view_data();
    ctx.insert("art", &art);
    ctx.insert("cate_data", &cate_data);

    ctx.insert("page", &page);
    ctx.insert("comments", &comments);
    ctx.insert("comments_count", &comments_count);
    ctx.insert("comments_num_pages", &comments_num_pages);

    if let Some(tags_string) = art.tags.clone() {
        let art_tags = tags_string.split(",").collect::<Vec<&str>>();
        ctx.insert("art_tags", &art_tags);
    }
  
    // 添加阅读量
    art::ArtModel::view_add(db, art.id, 1).await.unwrap_or_default();
  
    // 右侧数据
    let hot_arts = art::ArtModel::find_one_year_hot(db, 6).await.unwrap_or_default();
    let cates = cate::CateModel::find_open_cate(db).await.unwrap_or_default();
    let tags = tag::TagModel::find_open_tags(db, 6).await.unwrap_or_default();

    ctx.insert("hot_arts", &hot_arts);
    ctx.insert("cates", &cates);
    ctx.insert("tags", &tags);

    let mut tpl = cate_data.view_tpl.as_str();
    if tpl == "" {
        tpl = "view.html";
    }

    Ok(nako_http::view(&mut view, app::view_path(tpl).as_str(), &ctx))
}

