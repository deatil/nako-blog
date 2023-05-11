use actix_web::web;

use crate::app::controller::blog::{index};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                // 分组
                web::resource("/data").route(web::get().to(index::data)),
            )
            .service(
                // 返回 json
                web::resource("/json").route(web::get().to(index::json)),
            )
            .service(
                // 使用数据库
                web::resource("/usedb").route(web::get().to(index::usedb)),
            )
            .service(
                web::scope("/hello")
                    .service(
                        web::resource("/{name}").route(web::get().to(index::index)),
                    )
            ),
    );
}
