use actix_web::dev::{ServiceRequest, ServiceResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        let files = actix_files::Files::new("/", ".")
            .disable_content_disposition()
            .redirect_to_slash_directory()
            .index_file("index.html")
            .default_handler(|request: ServiceRequest| async {
                let (req, _) = request.into_parts();
                let res = match actix_files::NamedFile::open_async("./index.html").await {
                    Ok(file) => file.into_response(&req),
                    _ => actix_web::HttpResponse::NotFound().finish(),
                };
                Ok(ServiceResponse::new(req, res))
            });
        actix_web::App::new().service(files)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
