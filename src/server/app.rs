use actix_files::{
    Files,
    NamedFile,
};
use actix_web::{
    get,
    web,
    Result,
};

pub fn blog_config(config: &mut web::ServiceConfig) {
    config
    .service(blog_index)
    .service(blog_with_path)
    .service(blog_404);
}

#[get("/blog")]
async fn blog_index() -> Result<NamedFile> {
    Ok(NamedFile::open("./client/dist/index.html")?)
}

#[get("/blog/{path:.*}")]
async fn blog_with_path() -> Result<NamedFile> {
    Ok(NamedFile::open("./client/dist/index.html")?)
}

#[get("/404")]
async fn blog_404() -> Result<NamedFile> {
    Ok(NamedFile::open("./client/dist/index.html")?)
}

pub fn assets_config(config: &mut web::ServiceConfig) {
    config.service(assets());
}

fn assets() -> Files {
    Files::new("/assets", "./client/dist/assets")
}
