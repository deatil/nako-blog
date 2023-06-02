use actix_web::web;
use actix_web_lab::middleware::from_fn;

use crate::app::controller::blog::{
    error,
    index,
    cate,
    comment,
    page,
    tag,
    view,
    guestbook,
};

use crate::app::middleware::{
    blog_settings,
    blog_open,
};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                // 首页
                web::scope("/")
                    .service(
                        web::resource("")
                            .route(web::get().to(index::index))
                            .name("blog.index"),
                    ),
            )
            .service(
                // 分类
                web::scope("/c")
                    .service(
                        web::resource("")
                            .route(web::get().to(cate::index))
                            .name("blog.cate-index"),
                    )
                    .service(
                        web::resource("/{slug}")
                            .route(web::get().to(cate::name))
                            .name("blog.cate-name"),
                    ),
            )
            .service(
                // 详情
                web::scope("/a")
                    .service(
                        web::resource("/{uuid}")
                            .route(web::get().to(view::index))
                            .name("blog.view-index"),
                    )
            )
            .service(
                // 评论
                web::scope("/comment")
                    .service(
                        web::resource("/create")
                            .route(web::post().to(comment::create))
                            .name("blog.comment-create"),
                    ),
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
                // 页面
                web::scope("/p")
                    .service(
                        web::resource("/{slug}")
                            .route(web::get().to(page::index))
                            .name("blog.page-index"),
                    )
            )
            .service(
                // 评论
                web::scope("/guestbook")
                    .service(
                        web::resource("/create")
                            .route(web::post().to(guestbook::create))
                            .name("blog.guestbook-create"),
                    ),
            )
            .default_service(web::to(error::index))
            .wrap(from_fn(blog_settings::settings))
            .wrap(from_fn(blog_open::check)),
    );
}
