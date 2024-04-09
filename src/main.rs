#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new().service(actix_files::Files::new("/", ".").show_files_listing())
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
