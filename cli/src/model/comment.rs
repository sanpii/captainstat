#[derive(elephantry::Entity)]
pub struct Entity {
    pub comment_id: i32,
    pub approve: Option<bool>,
    pub inserted_at: chrono::DateTime<chrono::FixedOffset>,
    pub is_reported: bool,
    pub reply_to_id: Option<i32>,
    pub score: Option<i32>,
    pub source_url: Option<String>,
    pub statement_id: i32,
    pub text: Option<String>,
    pub user_id: Option<i32>,
}

pub struct Model;

impl elephantry::Model<'_> for Model {
    type Entity = Entity;
    type Structure = Structure;

    fn new(_: &elephantry::Connection) -> Self {
        Self
    }
}

pub struct Structure;

impl elephantry::Structure for Structure {
    fn relation() -> &'static str {
        "public.comment"
    }

    fn primary_key() -> &'static [&'static str] {
        &["id"]
    }

    fn columns() -> &'static [&'static str] {
        &[
            "comment_id",
            "approve",
            "inserted_at",
            "is_reported",
            "reply_to_id",
            "score",
            "source_url",
            "statement_id",
            "text",
            "user_id",
        ]
    }
}
