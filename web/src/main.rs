#![warn(rust_2018_idioms)]

mod errors;

use errors::*;

struct AppData {
    template: tera_hot::Template,
    elephantry: elephantry::Pool,
}

static TEMPLATE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let ip = std::env::var("LISTEN_IP").expect("Missing LISTEN_IP env variable");
    let port = std::env::var("LISTEN_PORT").expect("Missing LISTEN_IP env variable");
    let bind = format!("{}:{}", ip, port);

    let template = tera_hot::Template::new(TEMPLATE_DIR);
    template.clone().watch();

    actix_web::HttpServer::new(move || {
        let elephantry =
            elephantry::Pool::new(&database_url).expect("Unable to connect to postgresql");

        let data = AppData {
            template: template.clone(),
            elephantry,
        };

        let dir = format!("{}/static", env!("CARGO_MANIFEST_DIR"));
        let static_files = actix_files::Files::new("/static", &dir);

        actix_web::App::new()
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::normalize::TrailingSlash::Trim,
            ))
            .app_data(data)
            .service(index)
            .service(static_files)
    })
    .bind(&bind)?
    .run()
    .await
}

#[actix_web::get("/")]
async fn index(request: actix_web::HttpRequest) -> Result<actix_web::HttpResponse> {
    let data: &AppData = request.app_data()
        .unwrap();

    let body = data.template.render("index.html", &tera::Context::new())?;

    let response = actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(body);

    Ok(response)
}
