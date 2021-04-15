#[derive(elephantry::Entity)]
#[elephantry(model = "Model", structure = "Structure", relation = "public.user")]
pub struct Entity {
    #[elephantry(pk)]
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
