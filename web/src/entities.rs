#[derive(elephantry::Entity, serde::Deserialize, serde::Serialize)]
pub struct Video {
    title: String,
    url: String,
    thumbnail: String,
    percent_approves: f32,
    percent_refutes: f32,
    percent_comments: f32,
    nb_approves: i64,
    nb_refutes: i64,
    nb_comments: i64,
}

#[derive(elephantry::Entity, serde::Deserialize, serde::Serialize)]
pub struct Speaker {
    full_name: String,
    url: String,
    picture: Option<String>,
    percent_approves: f32,
    percent_refutes: f32,
    percent_comments: f32,
    nb_approves: i64,
    nb_refutes: i64,
    nb_comments: i64,
}
