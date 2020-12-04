#[derive(elephantry::Entity)]
pub struct Entity {
    pub video_id: i32,
    pub hash_id: String,
    pub inserted_at: chrono::DateTime<chrono::FixedOffset>,
    pub is_partner: bool,
    pub speaker_ids: Vec<i32>,
    pub thumbnail: String,
    pub title: String,
    pub youtube_id: String,
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
        "public.video"
    }

    fn primary_key() -> &'static [&'static str] {
        &["video_id"]
    }

    fn columns() -> &'static [&'static str] {
        &[
            "video_id",
            "hash_id",
            "inserted_at",
            "is_partner",
            "speaker_ids",
            "thumbnail",
            "title",
            "youtube_id",
        ]
    }
}
