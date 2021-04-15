#[derive(elephantry::Entity)]
#[elephantry(model = "Model", structure = "Structure", relation = "public.source")]
pub struct Entity {
    #[elephantry(pk)]
    pub url: String,
    pub language: Option<String>,
    pub site_name: Option<String>,
    pub title: Option<String>,
}
