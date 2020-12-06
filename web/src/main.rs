#![warn(rust_2018_idioms)]

mod errors;

use errors::*;

#[derive(elephantry::Entity, serde::Deserialize, serde::Serialize)]
pub struct Entity {
    title: String,
    url: String,
    picture: Option<String>,
    percent_approves: f32,
    percent_refutes: f32,
    percent_comments: f32,
    nb_approves: i64,
    nb_refutes: i64,
    nb_comments: i64,
}

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

    let mut template = tera_hot::Template::new(TEMPLATE_DIR);
    template.register_function("pager", elephantry_extras::tera::Pager);
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
            .service(videos)
            .service(speakers)
            .service(static_files)
    })
    .bind(&bind)?
    .run()
    .await
}

#[actix_web::get("/")]
async fn index(request: actix_web::HttpRequest) -> Result<actix_web::HttpResponse> {
    let data: &AppData = request.app_data().unwrap();

    let body = data.template.render("index.html", &tera::Context::new())?;

    let response = actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(body);

    Ok(response)
}

#[actix_web::get("/videos")]
async fn videos(
    request: actix_web::HttpRequest,
    pagination: actix_web::web::Query<elephantry_extras::Pagination>,
) -> Result<actix_web::HttpResponse> {
    let sql = include_str!("../sql/videos.sql");
    let sql_count = "select count(*) from video";
    list(sql, sql_count, &request, &pagination).await
}

#[actix_web::get("/speakers")]
async fn speakers(
    request: actix_web::HttpRequest,
    pagination: actix_web::web::Query<elephantry_extras::Pagination>,
) -> Result<actix_web::HttpResponse> {
    let sql = include_str!("../sql/speakers.sql");
    let sql_count = "select count(*) from speaker";
    list(sql, sql_count, &request, &pagination).await
}

async fn list(
    sql: &str,
    sql_count: &str,
    request: &actix_web::HttpRequest,
    pagination: &actix_web::web::Query<elephantry_extras::Pagination>,
) -> Result<actix_web::HttpResponse> {
    let data: &AppData = request.app_data().unwrap();

    let offset = ((pagination.page - 1) * pagination.limit) as u32;
    let limit = pagination.limit as u32;
    let entities = data.elephantry.query::<Entity>(sql, &[&offset, &limit])?;
    let count = data.elephantry.query_one::<i64>(sql_count, &[])?;
    let pager = elephantry::Pager::new(entities, count as usize, pagination.page, pagination.limit);
    let mut context = tera::Context::new();
    context.insert("pager", &pager);

    let body = data.template.render("list.html", &context)?;

    let response = actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(body);

    Ok(response)
}
