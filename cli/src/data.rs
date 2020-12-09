#[derive(serde::Deserialize)]
pub struct Data {
    pub data: Videos,
}

impl Data {
    pub fn hash_id(&self) -> Vec<&String> {
        self.data
            .videos
            .entries
            .iter()
            .map(|x| &x.hash_id)
            .collect()
    }

    pub fn total_page(&self) -> u32 {
        self.data.videos.total_pages
    }
}

#[derive(serde::Deserialize)]
pub struct Videos {
    pub videos: Entries,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entries {
    pub total_pages: u32,
    pub page_number: u32,
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

    pub fn statements(&self) -> Option<&Vec<Statement>> {
        for debate in &self.0 {
            if let Debate::Response(response) = debate {
                if let Content::Statements(ref statements) = response.response {
                    return Some(&statements);
                }
            }
        }

        None
    }

    pub fn comments(&self) -> Option<&Vec<Comment>> {
        for debate in &self.0 {
            if let Debate::Response(response) = debate {
                if let Content::Comments(ref comments) = response.response {
                    return Some(&comments.comments);
                }
            }
        }

        None
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Debate {
    String(Option<String>),
    Response(Response),
    PresenteState(PresenteState),
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
    Statements(Vec<Statement>),
    Comments(Comments),
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct PresenteState {
    viewers: Count,
    users: Count,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Count {
    count: usize,
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
            speaker_ids: self.speakers.iter().map(|x| x.id).collect(),
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

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Statement {
    pub id: i32,
    pub speaker_id: Option<i32>,
    #[serde(skip)]
    pub video_id: i32,
    pub text: String,
    pub time: i32,
}

impl std::convert::TryInto<crate::model::statement::Entity> for Statement {
    type Error = crate::Error;

    fn try_into(self) -> crate::Result<crate::model::statement::Entity> {
        let entity = crate::model::statement::Entity {
            statement_id: self.id,
            speaker_id: self.speaker_id,
            video_id: self.video_id,
            text: self.text,
            time: self.time,
        };

        Ok(entity)
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Comments {
    comments: Vec<Comment>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Comment {
    pub approve: Option<bool>,
    pub id: i32,
    pub inserted_at: String,
    pub is_reported: bool,
    pub reply_to_id: Option<i32>,
    pub score: Option<i32>,
    pub source: Option<Source>,
    pub statement_id: i32,
    pub text: Option<String>,
    pub user: Option<User>,
}

impl std::convert::TryInto<crate::model::comment::Entity> for Comment {
    type Error = crate::Error;

    fn try_into(self) -> crate::Result<crate::model::comment::Entity> {
        let entity = crate::model::comment::Entity {
            comment_id: self.id,
            approve: self.approve,
            inserted_at: chrono::DateTime::parse_from_str(
                &format!("{} +0000", self.inserted_at),
                "%Y-%m-%dT%H:%M:%S%.6f %z",
            )?,
            is_reported: self.is_reported,
            reply_to_id: self.reply_to_id,
            score: self.score,
            source_url: self.source.map(|x| x.url),
            statement_id: self.statement_id,
            text: self.text,
            user_id: self.user.map(|x| x.id),
        };

        Ok(entity)
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Source {
    language: Option<String>,
    site_name: Option<String>,
    title: Option<String>,
    url: String,
}

impl std::convert::TryInto<crate::model::source::Entity> for Source {
    type Error = crate::Error;

    fn try_into(self) -> crate::Result<crate::model::source::Entity> {
        let entity = crate::model::source::Entity {
            url: self.url,
            language: self.language,
            site_name: self.site_name,
            title: self.title,
        };

        Ok(entity)
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct User {
    achievements: Vec<i16>,
    id: i32,
    mini_picture_url: String,
    name: Option<String>,
    picture_url: String,
    registered_at: String,
    reputation: i32,
    speaker_id: Option<i32>,
    username: String,
}

impl std::convert::TryInto<crate::model::user::Entity> for User {
    type Error = crate::Error;

    fn try_into(self) -> crate::Result<crate::model::user::Entity> {
        let entity = crate::model::user::Entity {
            user_id: self.id,
            achievements: self.achievements,
            mini_picture_url: self.mini_picture_url,
            name: self.name,
            picture_url: self.picture_url,
            registered_at: chrono::DateTime::parse_from_str(
                &format!("{} +0000", self.registered_at),
                "%Y-%m-%dT%H:%M:%S%.6f %z",
            )?,
            reputation: self.reputation,
            speaker_id: self.speaker_id,
            username: self.username,
        };

        Ok(entity)
    }
}
