use actix_web::{web, error, Result, Error, Responder};
use actix_web_lab::respond::Html;

use crate::nako::global::{
    Session, AppState,
};

pub async fn index(
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

    let s = view.render("admin/index/index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(Html(s))
}