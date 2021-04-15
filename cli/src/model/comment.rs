#[derive(elephantry::Entity)]
#[elephantry(model = "Model", structure = "Structure", relation = "public.comment")]
pub struct Entity {
    #[elephantry(pk)]
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
