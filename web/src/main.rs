#![warn(warnings)]

mod errors;

use errors::*;

#[derive(elephantry::Entity, serde::Deserialize, serde::Serialize)]
pub struct Entity {
    title: String,
    url: String,
    picture: Option<String>,
    #[elephantry(default)]
    percent_approves: f32,
    #[elephantry(default)]
    percent_refutes: f32,
    #[elephantry(default)]
    percent_comments: f32,
    #[elephantry(default)]
    nb_approves: i64,
    #[elephantry(default)]
    nb_refutes: i64,
    #[elephantry(default)]
    nb_comments: i64,
}

#[derive(serde::Deserialize)]
struct Query {
    q: String,
    #[serde(flatten)]
    pagination: elephantry_extras::Pagination,
}

struct Data {
    template: tera_hot::Template,
    elephantry: elephantry::Pool,
}

static TEMPLATE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let ip = std::env::var("LISTEN_IP").expect("Missing LISTEN_IP env variable");
    let port = std::env::var("LISTEN_PORT").expect("Missing LISTEN_IP env variable");
    let bind = format!("{}:{}", ip, port);

    let mut template = tera_hot::Template::new(TEMPLATE_DIR);
    template.register_function("pager", elephantry_extras::tera::Pager);
    template.clone().watch();

    let elephantry = elephantry::Pool::new(&database_url).expect("Unable to connect to postgresql");

    actix_web::HttpServer::new(move || {
        let data = Data {
            elephantry: elephantry.clone(),
            template: template.clone(),
        };

        let dir = format!("{}/static", env!("CARGO_MANIFEST_DIR"));
        let static_files = actix_files::Files::new("/static", &dir);

        actix_web::App::new()
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .app_data(actix_web::web::Data::new(data))
            .service(index)
            .service(video)
            .service(speaker)
            .service(user)
            .service(search_video)
            .service(search_speaker)
            .service(search_user)
            .service(static_files)
    })
    .bind(&bind)?
    .run()
    .await
}

#[actix_web::get("/")]
async fn index(data: actix_web::web::Data<Data>) -> Result<actix_web::HttpResponse> {
    let body = data.template.render("index.html", &tera::Context::new())?;

    let response = actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(body);

    Ok(response)
}

macro_rules! route {
    ($list_name:ident, $list_url:literal, $search_name:ident, $search_url:literal) => {
        #[actix_web::get($list_url)]
        async fn $list_name(
            data: actix_web::web::Data<Data>,
            pagination: actix_web::web::Query<elephantry_extras::Pagination>,
        ) -> Result<actix_web::HttpResponse> {
            let sql = format!("select * from view.{}", stringify!($list_name));
            $crate::list($list_url, &sql, None, &data, &pagination)
        }

        #[actix_web::get($search_url)]
        async fn $search_name(
            data: actix_web::web::Data<Data>,
            query: actix_web::web::Query<Query>,
        ) -> Result<actix_web::HttpResponse> {
            let sql = $crate::search_query(stringify!($list_name));
            list(
                &format!("{}?q={}", $search_url, query.q),
                &sql,
                Some(&query.q),
                &data,
                &query.pagination,
            )
        }
    };
}

route!(video, "/videos", search_video, "/search/videos");
route!(speaker, "/speakers", search_speaker, "/search/speakers");
route!(user, "/users", search_user, "/search/users");

fn search_query(ty: &str) -> String {
    if ty == "video" {
        format!(
            "
select view.{ty}.*
    from websearch_to_tsquery('french', $1) query,
        view.{ty}
    where view.{ty}.document @@ query
        or url = $1
    order by ts_rank_cd(view.{ty}.document, query) desc
",
            ty = ty
        )
    } else {
        format!(
            "
select view.{ty}.*
    from view.{ty}
    where view.{ty}.title ~* $1
        or url = $1
    order by view.{ty}.title
",
            ty = ty
        )
    }
}

fn list(
    base_url: &str,
    sql: &str,
    q: Option<&str>,
    data: &Data,
    pagination: &elephantry_extras::Pagination,
) -> Result<actix_web::HttpResponse> {
    let pager = query(&data.elephantry, sql, q, pagination)?;

    render(&data.template, &pager, base_url, q)
}

fn query(
    elephantry: &elephantry::Connection,
    sql: &str,
    q: Option<&str>,
    pagination: &elephantry_extras::Pagination,
) -> Result<elephantry::Pager<Entity>> {
    let paginate_sql = format!("{} {}", sql, pagination.to_sql(),);

    let sql_count = format!("with query as ({}) select count(1) from query", sql,);

    let params = if q.is_some() {
        vec![&q as &dyn elephantry::ToSql]
    } else {
        Vec::new()
    };

    let entities = elephantry.query::<Entity>(&paginate_sql, &params)?;
    let count = elephantry.query_one::<i64>(&sql_count, &params)?;

    let pager = elephantry::Pager::new(entities, count as usize, pagination.page, pagination.limit);

    Ok(pager)
}

fn render(
    template: &tera_hot::Template,
    pager: &elephantry::Pager<Entity>,
    base_url: &str,
    q: Option<&str>,
) -> Result<actix_web::HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("pager", &pager);
    context.insert("base_url", &base_url);
    if q.is_some() {
        context.insert("q", &q);
    }

    let body = template.render("list.html", &context)?;

    let response = actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(body);

    Ok(response)
}
