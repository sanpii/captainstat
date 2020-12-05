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
    let data = get_data(&token)?;

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let elephantry = elephantry::Pool::new(&database_url)?;

    for ref hash_id in data.hash_id() {
        let debates = get_debates(&hash_id, &token)?;

        let video = match debates.video() {
            Some(video) => video,
            None => continue,
        };

        if let Err(err) = save::<model::video::Model, _>(&elephantry, "video_pkey", video) {
            log::error!("Unable to save video: {}", err);
        }

        for speaker in &video.speakers {
            if let Err(err) = save::<model::speaker::Model, _>(&elephantry, "speaker_pkey", speaker)
            {
                log::error!("Unable to save speaker: {}", err);
            }
        }
    }

    Ok(())
}

fn get_data(token: &str) -> Result<Data> {
    let query = serde_json::json!({
        "operationName" : "VideosIndex",
        "query" : "query VideosIndex($offset: Int! = 1, $limit: Int! = 16, $filters: VideoFilter = {}) {
            videos(limit: $limit, offset: $offset, filters: $filters) {
                pageNumber
                totalPages
                entries {
                    hash_id: hashId
                    __typename
                }
                __typename
            }
        }",
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

    Ok(response)
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

fn get_debates(id: &str, token: &str) -> Result<data::Debates> {
    let request = format!(r#"["1","1","video_debate:{}","phx_join",{{}}]"#, id);
    dbg!(&request);

    websocket(request, token)
}

fn websocket(request: String, token: &str) -> Result<data::Debates> {
    let url = format!(
        "wss://api.captainfact.io/socket/websocket?token={}&vsn=2.0.0",
        token
    );
    let (mut socket, _) = tungstenite::connect(&url)?;

    socket.write_message(tungstenite::Message::Text(request))?;

    let response = socket.read_message()?;
    let debates = serde_json::from_str(response.to_text()?)?;

    Ok(debates)
}
