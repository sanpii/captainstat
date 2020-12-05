#[derive(serde::Deserialize)]
pub struct Data {
    pub data: Videos,
}

impl Data {
    pub fn hash_id(&self) -> Vec<&String> {
        self.data.videos.entries.iter().map(|x| &x.hash_id).collect()
    }
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
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Debates(Vec<Debate>);

impl Debates {
    pub fn video(&self) -> Option<&DebateVideo> {
        for debate in &self.0 {
            if let Debate::Response(response) = debate {
                if let Content::Video(ref video) = response.response {
                    return Some(&video);
                }
            }
        }

        None
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Debate {
    String(String),
    Response(Response),
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Response {
    pub response: Content,
    pub status: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Content {
    Video(DebateVideo),
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DebateVideo {
    pub hash_id: String,
    pub id: i32,
    pub is_partner: bool,
    pub is_subscribed: bool,
    pub language: Option<String>,
    pub posted_at: String,
    pub provider: String,
    pub provider_id: Option<String>,
    pub speakers: Vec<Speaker>,
    pub thumbnail: String,
    pub title: String,
    pub url: String,
    pub youtube_id: Option<String>,
    pub youtube_offset: i32,
}

impl std::convert::TryInto<crate::model::video::Entity> for DebateVideo {
    type Error = crate::Error;

    fn try_into(self) -> crate::Result<crate::model::video::Entity> {
        let entity = crate::model::video::Entity {
            video_id: self.id,
            hash_id: self.hash_id,
            posted_at: self.posted_at.parse()?,
            is_partner: self.is_partner,
            speaker_ids: self
                .speakers
                .iter()
                .map(|x| x.id)
                .collect(),
            thumbnail: self.thumbnail,
           title: self.title,
            youtube_id: self.youtube_id,
            language: self.language,
            url: self.url,
            youtube_offset: self.youtube_offset,
            provider: self.provider,
            provider_id: self.provider_id,
        };

        Ok(entity)
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Speaker {
    pub country: Option<String>,
    pub full_name: String,
    pub id: i32,
    pub picture: Option<String>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub wikidata_item_id: Option<String>,
}

impl std::convert::TryInto<crate::model::speaker::Entity> for Speaker {
    type Error = crate::Error;

    fn try_into(self) -> crate::Result<crate::model::speaker::Entity> {
        let entity = crate::model::speaker::Entity {
            speaker_id: self.id,
            full_name: self.full_name,
            slug: self.slug,
            country: self.country,
            picture: self.picture,
            title: self.title,
            wikidata_item_id: self.wikidata_item_id,
        };

        Ok(entity)
    }
}
