#[derive(elephantry::Entity)]
pub struct Entity {
    pub speaker_id: i32,
    pub full_name: String,
    pub slug: Option<String>,
    pub country: Option<String>,
    pub picture: Option<String>,
    pub title: Option<String>,
    pub wikidata_item_id: Option<String>,
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
        "public.speaker"
    }

    fn primary_key() -> &'static [&'static str] {
        &["speaker_id"]
    }

    fn columns() -> &'static [&'static str] {
        &[
            "speaker_id",
            "full_name",
            "slug",
            "country",
            "picture",
            "title",
            "wikidata_item_id",
        ]
    }
}
