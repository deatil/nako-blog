use actix_session::{
    SessionMiddleware,
    storage::RedisSessionStore, 
    config::PersistentSession,
};
use actix_web::{
    web, 
    App, 
    Error,
    Result,
    HttpServer,
    dev::Service,
    http::{
        StatusCode,
    },
    cookie::{
        Key,
        time,
    },
    middleware::{
        Logger,
        ErrorHandlers,
    },
    HttpResponse,
};
use actix_files::Files as Fs;

use tera::Tera;
use mime_guess::from_path;
use listenfd::ListenFd;

use crate::nako::{
    db, 
    embed,
    config,
    redis,
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
    let rust_log = config::section::<String>("app", "rust_log", "error".to_string());
    std::env::set_var("RUST_LOG", rust_log.as_str());

    // 导入环境变量
    dotenvy::dotenv().ok();

    // 日志
    let logger = nako_log::setup_logger();
    match logger {
        Ok(_) => {},
        Err(err) => log::error!("set log err: {err}"),
    }

    let host = config::section::<String>("server", "host", "127.0.0.1".to_string());
    let port = config::section::<String>("server", "port", "8080".to_string());
    let server_url = format!("{host}:{port}");

    let conn = db::connect().await.unwrap_or_default();

    let mut view: Tera;
 
    // 是否打包
    let is_embed = config::section::<bool>("app", "is_embed", true);
    if is_embed {
        view = Tera::default();

        for file in embed::Templates::iter() {
            let filename = file.as_ref();
            view.add_raw_template(filename.clone(), embed::get_tpl_data(filename.clone()).as_str()).unwrap_or_default();
        }
    } else {
        view = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assert/templates/**/*")).unwrap_or_default();
    }

    // 设置模板函数
    nako_view::set_fns(&mut view);

    let redis_url = config::section::<String>("redis", "url", "8080".to_string());
    let redis = redis::create_redis_pool(redis_url).await.unwrap();

    let state = AppState { 
        view: view, 
        db: conn, 
        redis: redis, 
    };

    let session_redis_url = config::section::<String>("session", "redis_url", "redis://127.0.0.1:6379".to_string());
    let redis_store = RedisSessionStore::new(session_redis_url.clone()).await.unwrap();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(
                ErrorHandlers::new().handler(StatusCode::NOT_FOUND, error::not_found)
            )
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    redis_store.clone(),
                    Key::from(&[0;64]),
                )
                .cookie_secure(false)
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(time::Duration::days(5))
                )
                .build(),
            )
            .app_data(
                web::Data::new(state.clone()),
            )
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
            .wrap_fn(move |req, srv| {
                nako_view::ROUTES_KEY.with(|routes| {
                    routes.borrow_mut().replace(req.resource_map().clone());
                });
                
                srv.call(req)
            })
            .service(Fs::new("/upload", "./storage/upload"))
            .configure(|cfg: &mut web::ServiceConfig| {
                let is_embed = config::section::<bool>("app", "is_embed", true);
                if is_embed {
                    cfg.service(handle_embedded_static);
                } else {
                    cfg.service(Fs::new("/static", "./assert/static"));
                }
            })
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

/// 静态资源
#[actix_web::get("/static/{_:.*}")]
async fn handle_embedded_static(
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let path = path.as_str();

    match embed::Static::get(path) {
        Some(content) => Ok(HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned())),
        None => Ok(HttpResponse::NotFound().body("404 Not Found")),
    }
}
