#[derive(elephantry::Entity)]
#[elephantry(model = "Model", structure = "Structure", relation = "public.video")]
pub struct Entity {
    #[elephantry(pk)]
    pub video_id: i32,
    pub hash_id: String,
    pub posted_at: chrono::DateTime<chrono::FixedOffset>,
    pub is_partner: bool,
    pub speaker_ids: Vec<i32>,
    pub thumbnail: String,
    pub title: String,
    pub youtube_id: Option<String>,
    pub language: Option<String>,
    pub url: String,
    pub youtube_offset: i32,
    pub provider: String,
    pub provider_id: Option<String>,
}
