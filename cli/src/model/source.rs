#[derive(elephantry::Entity)]
pub struct Entity {
    pub url: String,
    pub language: Option<String>,
    pub site_name: Option<String>,
    pub title: Option<String>,
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
        "public.source"
    }

    fn primary_key() -> &'static [&'static str] {
        &["url"]
    }

    fn columns() -> &'static [&'static str] {
        &["url", "language", "site_name", "title"]
    }
}
