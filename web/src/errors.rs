pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Elephantry(#[from] elephantry::Error),
    #[error("")]
    NotFound,
    #[error("{0}")]
    Template(#[from] tera::Error),
}

impl From<&Error> for actix_web::http::StatusCode {
    fn from(error: &Error) -> Self {
        use actix_web::http::StatusCode;

        match error {
            Error::Elephantry(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        let status: actix_web::http::StatusCode = self.into();

        let file = format!("errors/{}.html", u16::from(status));
        let template = tera_hot::Template::new(crate::TEMPLATE_DIR);
        let body = match template.render(&file, &tera::Context::new()) {
            Ok(body) => body,
            Err(err) => {
                eprintln!("{:?}", err);

                "Internal server error".to_string()
            }
        };

        actix_web::HttpResponse::build(status)
            .append_header((actix_web::http::header::CONTENT_TYPE, "text/html"))
            .body(body)
    }
}
