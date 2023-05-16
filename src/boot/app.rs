use std::env;

use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, SessionMiddleware,
};
use actix_web::{
    cookie::{time::Duration, Key},
    web, App, HttpServer
};
use actix_web::middleware::Logger;
use actix_files::Files as Fs;

use tera::Tera;
use listenfd::ListenFd;

use crate::nako::{
    log as nako_log,
    global::AppState,
    db, 
    view as nako_view
};

use crate::boot::{error};
use crate::route::{
    admin,
    blog,
};

/// app 运行
pub async fn start() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    // 日志
    let logger = nako_log::setup_logger();
    match logger {
        Ok(_) => {},
        Err(err) => log::error!("set log err: {err}"),
    }

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = db::connect().await.unwrap();
    let mut view = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assert/templates/**/*")).unwrap();

    // 设置模板函数
    view.register_function("assert", nako_view::assert);

    let state = AppState { 
        view: view, 
        db: conn, 
    };

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .service(Fs::new("/static", "./assert/static"))
            .service(Fs::new("/upload", "./assert/upload"))
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::hours(2)),
                    )
                    .build(),
            )
            .configure(admin::route)
            .configure(blog::route)
            .service(web::scope("").wrap(error::error_handlers()))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await
}
