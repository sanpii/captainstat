#[derive(elephantry::Entity)]
#[elephantry(model = "Model", structure = "Structure", relation = "public.speaker")]
pub struct Entity {
    #[elephantry(pk)]
    pub speaker_id: i32,
    pub full_name: String,
    pub slug: Option<String>,
    pub country: Option<String>,
    pub picture: Option<String>,
    pub title: Option<String>,
    pub wikidata_item_id: Option<String>,
}
