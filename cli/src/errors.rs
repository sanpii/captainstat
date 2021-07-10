pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to login ({0}): {1}")]
    Auth(attohttpc::StatusCode, serde_json::Value),

    #[error("{0}")]
    Database(#[from] elephantry::Error),

    #[error("{0}")]
    Date(#[from] chrono::ParseError),

    #[error("Missing {0} environment variable")]
    Env(String),

    #[error("{0}")]
    Integer(#[from] std::num::ParseIntError),

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Http(#[from] attohttpc::Error),

    #[error("{0}")]
    Websocket(#[from] tungstenite::Error),

    #[error("Maximum tries reach for websocket")]
    WebsocketTryOut,
}
