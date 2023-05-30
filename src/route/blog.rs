use actix_web::web;

use crate::app::controller::blog::{
    error,
    index,
    cate,
    comment,
    page,
    tag,
    view,
};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::scope("/index")
                    .service(
                        web::resource("")
                            .route(web::get().to(index::index))
                            .name("blog.index"),
                    ),
            )
            .service(
                // 分类
                web::scope("/cate")
                    .service(
                        web::resource("")
                            .route(web::get().to(cate::index))
                            .name("blog.cate-index"),
                    )
                    .service(
                        web::resource("/{name}")
                            .route(web::get().to(cate::name))
                            .name("blog.cate-name"),
                    ),
            )
            .service(
                // 评论
                web::scope("/comment")
                    .service(
                        web::resource("/{art_id}")
                            .route(web::get().to(comment::index))
                            .name("blog.comment-index"),
                    )
                    .service(
                        web::resource("/create")
                            .route(web::post().to(comment::create))
                            .name("blog.comment-create"),
                    ),
            )
            .service(
                // 页面
                web::scope("/page")
                    .service(
                        web::resource("/{slug}")
                            .route(web::get().to(page::index))
                            .name("blog.page-index"),
                    )
            )
            .service(
                // 标签
                web::scope("/tag")
                    .service(
                        web::resource("/{name}")
                            .route(web::get().to(tag::index))
                            .name("blog.tag-index"),
                    )
            )
            .service(
                // 详情
                web::scope("/view")
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(view::index))
                            .name("blog.view-index"),
                    )
            )
            .default_service(web::to(error::index)),
    );
}
