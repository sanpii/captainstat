mod data;
mod errors;
mod model;

use data::*;
use errors::*;
use std::convert::TryInto;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    pretty_env_logger::init();

    let token = std::env::var("TOKEN").expect("Missing TOKEN env variable");
    let videos = get_videos(&token)?;

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let elephantry = elephantry::Pool::new(&database_url)?;

    for ref video in videos {
        if let Err(err) = save::<model::video::Model, _>(&elephantry, "video_pkey", video) {
            log::error!("Unable to save video: {}", err);
        }

        for speaker in &video.speakers {
            if let Err(err) = save::<model::speaker::Model, _>(&elephantry, "speaker_pkey", speaker) {
                log::error!("Unable to save speaker: {}", err);
            }
        }
    }

    Ok(())
}

fn get_videos(token: &str) -> Result<Vec<Video>> {
    let query = serde_json::json!({
        "operationName" : "VideosIndex",
        "query" : "query VideosIndex($offset: Int! = 1, $limit: Int! = 16, $filters: VideoFilter = {}) {\n  videos(limit: $limit, offset: $offset, filters: $filters) {\n pageNumber\n    totalPages\n    entries {\n      id\n      hash_id: hashId\n youtube_id: youtubeId\n      title\n      insertedAt\n      isPartner\n thumbnail\n      speakers {\n        full_name: fullName\n        id\n slug\n        __typename\n      }\n      __typename\n    }\n    __typename\n }\n}\n",
        "variables" : {
            "filters" : {},
            "limit" : 4,
            "offset" : 1
        }
    });
    let response: Data = attohttpc::post("https://graphql.captainfact.io")
        .header("authorization", format!("Bearer {}", token))
        .json(&query)?
        .send()?
        .json()?;

    Ok(response.data.videos.entries)
}

fn save<'a, M, T>(elephantry: &elephantry::Connection, constraint: &str, data: &T) -> Result<()>
where
    M: elephantry::Model<'a>,
    T: TryInto<M::Entity, Error = crate::Error> + Clone,
{
    let entity = data.clone().try_into()?;

    elephantry.upsert_one::<M>(&entity, &format!("on constraint {}", constraint), "nothing")?;

    Ok(())
}
