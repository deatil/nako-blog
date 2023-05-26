use actix_web::web;
use actix_web_lab::middleware::from_fn;

use crate::nako::{
    env,
};

use crate::app::middleware::{admin_auth};
use crate::app::controller::admin::{
    index,
    auth,
    user,
    profile,
    upload,
    attach,
};

pub fn route(cfg: &mut web::ServiceConfig) {
    let admin_prefix = env::get_env::<String>("ADMIN_PREFIX", "admin".to_string());

    cfg.service(
        web::scope(admin_prefix.as_str())
            .service(
                // 后台首页
                web::scope("/index")
                    .service(
                        web::resource("")
                            .route(web::get().to(index::index))
                            .name("admin.index"),
                    )
                    .service(
                        web::resource("/menu")
                            .route(web::get().to(index::menu))
                            .name("admin.index-menu"),
                    )
                    .service(
                        web::resource("/console")
                            .route(web::get().to(index::console))
                            .name("admin.index-console"),
                    )
            )
            .service(
                // 登陆相关
                web::scope("/auth")
                    .service(
                        web::resource("/captcha")
                            .route(web::get().to(auth::captcha))
                            .name("admin.auth-captcha"),
                    )
                    .service(
                        web::resource("/login")
                            .route(web::get().to(auth::login))
                            .route(web::post().to(auth::login_check))
                            .name("admin.auth-login"),
                    )
                    .service(
                        web::resource("/logout")
                            .route(web::get().to(auth::logout))
                            .name("admin.auth-logout"),
                    )
            )
            .service(
                // 个人信息
                web::scope("/profile")
                    .service(
                        web::resource("/info")
                            .route(web::get().to(profile::update_info))
                            .route(web::post().to(profile::update_info_save))
                            .name("admin.profile-info"),
                    )
                    .service(
                        web::resource("/password")
                            .route(web::get().to(profile::update_password))
                            .route(web::post().to(profile::update_password_save))
                            .name("admin.profile-password"),
                    )
                    .service(
                        web::resource("/avatar")
                            .route(web::get().to(profile::update_avatar))
                            .route(web::post().to(profile::update_avatar_save))
                            .name("admin.profile-avatar"),
                    )
            )
            .service(
                // 上传
                web::scope("/upload")
                    .service(
                        web::resource("/file")
                            .route(web::post().to(upload::file))
                            .name("admin.upload-file"),
                    )
                    .service(
                        web::resource("/avatar")
                            .route(web::post().to(upload::avatar))
                            .name("admin.upload-avatar"),
                    )
            )
            .service(
                // 用户
                web::scope("/user")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(user::index))
                            .name("admin.user-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(user::list))
                            .name("admin.user-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(user::detail))
                            .name("admin.user-detail"),
                    )
                    .service(
                        web::resource("/create")
                            .route(web::get().to(user::create))
                            .route(web::post().to(user::create_save))
                            .name("admin.user-create"),
                    )
                    .service(
                        web::resource("/update")
                            .route(web::get().to(user::update))
                            .route(web::post().to(user::update_save))
                            .name("admin.user-update"),
                    )
                    .service(
                        web::resource("/status")
                            .route(web::post().to(user::update_status))
                            .name("admin.user-status"),
                    )
                    .service(
                        web::resource("/update-password")
                            .route(web::get().to(user::update_password))
                            .route(web::post().to(user::update_password_save))
                            .name("admin.user-update-password"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(user::delete))
                            .name("admin.user-delete"),
                    )
            )
            .service(
                // 附件
                web::scope("/attach")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(attach::index))
                            .name("admin.attach-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(attach::list))
                            .name("admin.attach-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(attach::detail))
                            .name("admin.attach-detail"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(attach::delete))
                            .name("admin.attach-delete"),
                    )
                    .service(
                        web::resource("/download")
                            .route(web::get().to(attach::download))
                            .name("admin.attach-download"),
                    )
            )            
            .wrap(from_fn(admin_auth::auth)),
    );
}
