#[derive(serde::Deserialize)]
pub struct Data {
    pub data: Videos,
}

#[derive(serde::Deserialize)]
pub struct Videos {
    pub videos: Entries,
}

#[derive(serde::Deserialize)]
pub struct Entries {
    pub entries: Vec<Video>,
}

#[derive(Clone, serde::Deserialize)]
pub struct Video {
    pub hash_id: String,
    pub id: String,
    #[serde(rename = "insertedAt")]
    pub inserted_at: String,
    #[serde(rename = "isPartner")]
    pub is_partner: bool,
    pub speakers: Vec<Speaker>,
    pub thumbnail: String,
    pub title: String,
    pub youtube_id: String,
}

impl std::convert::TryInto<crate::model::video::Entity> for Video {
    type Error = crate::Error;

    fn try_into(self) -> crate::Result<crate::model::video::Entity> {
        let entity = crate::model::video::Entity {
            video_id: self.id.parse()?,
            hash_id: self.hash_id,
            inserted_at: self.inserted_at.parse()?,
            is_partner: self.is_partner,
            speaker_ids: self
                .speakers
                .iter()
                .map(|x| x.id.parse().unwrap())
                .collect(),
            thumbnail: self.thumbnail,
            title: self.title,
            youtube_id: self.youtube_id,
        };

        Ok(entity)
    }
}

#[derive(Clone, serde::Deserialize)]
pub struct Speaker {
    pub full_name: String,
    pub id: String,
    pub slug: Option<String>,
}

impl std::convert::TryInto<crate::model::speaker::Entity> for Speaker {
    type Error = crate::Error;

    fn try_into(self) -> crate::Result<crate::model::speaker::Entity> {
        let entity = crate::model::speaker::Entity {
            speaker_id: self.id.parse()?,
            full_name: self.full_name,
            slug: self.slug,
        };

        Ok(entity)
    }
}
