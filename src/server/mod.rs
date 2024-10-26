mod api;
mod app;

use actix_web::{
    web,
    App,
    HttpServer,
};

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(web::redirect("/", "/blog"))
        .configure(api::api_config)
        .configure(app::blog_config)
        .configure(app::assets_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
