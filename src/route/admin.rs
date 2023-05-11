use actix_web::web;
use actix_web_lab::middleware::from_fn;

use crate::app::middleware::{admin_auth};
use crate::app::controller::admin::{
    index,
    auth,
};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("admin")
            .service(
                // 后台首页
                web::resource("/index").route(web::get().to(index::index)).name("admin.index"),
            )
            .service(
                // 登陆相关
                web::scope("/auth")
                    .service(
                        web::resource("/captcha").route(web::get().to(auth::captcha)).name("admin.auth-captcha"),
                    )
                    .service(
                        web::resource("/login").route(web::get().to(auth::login)).name("admin.auth-login"),
                    )
                    .service(
                        web::resource("/login").route(web::post().to(auth::login_check)).name("admin.auth-login-check"),
                    )
                    .service(
                        web::resource("/logout").route(web::get().to(auth::logout)).name("admin.auth-logout"),
                    )
            )
            .wrap(from_fn(admin_auth::auth)),
    );
}
