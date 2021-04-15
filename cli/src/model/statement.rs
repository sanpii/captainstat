#[derive(elephantry::Entity)]
#[elephantry(
    model = "Model",
    structure = "Structure",
    relation = "public.statement"
)]
pub struct Entity {
    pub statement_id: i32,
    pub speaker_id: Option<i32>,
    pub video_id: i32,
    pub text: String,
    pub time: i32,
}
