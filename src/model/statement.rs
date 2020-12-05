#[derive(elephantry::Entity)]
pub struct Entity {
    pub statement_id: i32,
    pub speaker_id: Option<i32>,
    pub video_id: i32,
    pub text: String,
    pub time: i32,
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
        "public.statement"
    }

    fn primary_key() -> &'static [&'static str] {
        &["statement_id"]
    }

    fn columns() -> &'static [&'static str] {
        &[
            "statement_id",
            "video_id",
            "speaker_id",
            "text",
            "time",
        ]
    }
}
