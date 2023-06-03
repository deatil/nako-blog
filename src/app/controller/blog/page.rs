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
    page,
};

/// 单页
pub async fn index(
    state: web::Data<AppState>,
    slug: web::Path<String>
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    // 页面详情
    let data = page::PageModel::find_by_slug(db, slug.as_str())
        .await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(app::error_html(&mut view, "文章不存在"));
    }

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &data);

    let tpl = match data.tpl {
        Some(v) => v,
        _ => "page.html".into(),
    };

    Ok(nako_http::view(&mut view, app::view_path(tpl.as_str()).as_str(), &ctx))
}
