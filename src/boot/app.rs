use actix_session::{
    SessionMiddleware,
    config::PersistentSession, 
    storage::CookieSessionStore, 
};
use actix_web::{
    web, 
    App, 
    HttpServer,
    http::{
        StatusCode,
    },
    cookie::{
        Key,
        time::Duration, 
    },
    middleware::{
        ErrorHandlers,
    },
};
use actix_web::middleware::Logger;
use actix_files::Files as Fs;

use tera::Tera;
use listenfd::ListenFd;

use crate::nako::{
    db, 
    env,
    view as nako_view,
    log as nako_log,
    global::AppState,
};

use crate::boot::{
    error,
};
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

    let host = env::get_env::<String>("HOST", "127.0.0.1".to_string());
    let port = env::get_env::<String>("PORT", "8080".to_string());
    let server_url = format!("{host}:{port}");

    let conn = db::connect().await.unwrap_or_default();
    let mut view = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assert/templates/**/*")).unwrap_or_default();

    // 设置模板函数
    nako_view::set_fns(&mut view);

    let state = AppState { 
        view: view, 
        db: conn, 
    };

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(
                ErrorHandlers::new().handler(StatusCode::NOT_FOUND, error::not_found)
            )
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::hours(2)),
                    )
                    .build(),
            )
            .app_data(web::Data::new(state.clone()))
            .app_data(
                web::FormConfig::default()
                    .error_handler(error::form_parser_error)
                    .clone(),
            )
            .app_data(
                web::JsonConfig::default()
                    .error_handler(error::json_parser_error)
                    .clone(),
            )
            .app_data(
                web::QueryConfig::default()
                    .error_handler(error::query_parser_error)
                    .clone(),
            )
            .app_data(
                web::PathConfig::default()
                    .error_handler(error::path_parser_error)
                    .clone(),
            )
            .service(Fs::new("/static", "./assert/static"))
            .service(Fs::new("/upload", "./assert/upload"))
            .configure(admin::route)
            .configure(blog::route)
            .default_service(web::to(error::app_default))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await
}
