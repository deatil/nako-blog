use nako_blog::boot;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    boot::app::start().await
}