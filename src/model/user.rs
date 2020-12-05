#[derive(elephantry::Entity)]
pub struct Entity {
    pub user_id: i32,
    pub achievements: Vec<i16>,
    pub mini_picture_url: String,
    pub name: Option<String>,
    pub picture_url: String,
    pub registered_at: chrono::DateTime<chrono::FixedOffset>,
    pub reputation: i32,
    pub speaker_id: Option<i32>,
    pub username: String,
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
        "public.user"
    }

    fn primary_key() -> &'static [&'static str] {
        &["user_id"]
    }

    fn columns() -> &'static [&'static str] {
        &[
            "user_id",
            "achievements",
            "mini_picture_url",
            "name",
            "picture_url",
            "registered_at",
            "reputation",
            "speaker_id",
            "username",
        ]
    }
}
