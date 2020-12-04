#[derive(Debug, serde::Deserialize)]
pub struct Data {
    pub data: Videos,
}

#[derive(Debug, serde::Deserialize)]
pub struct Videos {
    pub videos: Entries,
}

#[derive(Debug, serde::Deserialize)]
pub struct Entries {
    pub entries: Vec<Video>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Video {
    hash_id: String,
    id: String,
    #[serde(rename="insertedAt")]
    inserted_at: String,
    #[serde(rename="isPartner")]
    is_partner: bool,
    speakers: Vec<Speaker>,
    thumbnail: String,
    title: String,
    youtube_id: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Speaker {
    full_name: String,
    id: String,
    slug: Option<String>,
}
