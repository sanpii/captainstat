pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Database(#[from] elephantry::Error),

    #[error("{0}")]
    Date(#[from] chrono::ParseError),

    #[error("{0}")]
    Integer(#[from] std::num::ParseIntError),

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Http(#[from] attohttpc::Error),

    #[error("{0}")]
    Websocket(#[from] tungstenite::Error),
}
