mod data;
mod errors;
mod model;

use data::*;
use errors::*;
use std::convert::TryInto;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    video_hash_id: Option<String>,
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    pretty_env_logger::init();

    let opt = Opt::from_args();

    let token = std::env::var("TOKEN").expect("Missing TOKEN env variable");

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let elephantry = elephantry::Pool::new(&database_url)?;

    if let Some(video_hash_id) = opt.video_hash_id {
        save_video(&elephantry, &token, &video_hash_id)?;
    } else {
        let mut page = 1;

        loop {
            let data = get_data(&token, page)?;

            log::info!("Fetching page {}/{}", page, data.total_page());

            for ref hash_id in data.hash_id() {
                if save_video(&elephantry, &token, hash_id).is_err() {
                    log::error!("Unable to save video '{}'", hash_id);
                }
            }

            if page == data.total_page() {
                break;
            } else {
                page += 1;
            }
        }
    }

    Ok(())
}

fn get_data(token: &str, page: u32) -> Result<Data> {
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
            "limit" : 16,
            "offset" : page,
        }
    });

    let response: Data = attohttpc::post("https://graphql.captainfact.io")
        .header("authorization", format!("Bearer {}", token))
        .json(&query)?
        .send()?
        .json()?;

    Ok(response)
}

fn save_video(elephantry: &elephantry::Connection, token: &str, hash_id: &str) -> Result<()> {
    let debates = get_debates(&hash_id, token)?;
    let video = match debates.video() {
        Some(video) => video,
        None => return Ok(()),
    };

    save::<model::video::Model, _>(&elephantry, "video_pkey", video)?;

    for speaker in &video.speakers {
        save::<model::speaker::Model, _>(&elephantry, "speaker_pkey", speaker)?;
    }

    let statements = get_statements(hash_id, token)?;
    let statements = match statements.statements() {
        Some(statements) => statements,
        None => return Ok(()),
    };

    for statement in statements {
        let mut st = statement.clone();
        st.video_id = video.id;

        save::<model::statement::Model, _>(&elephantry, "statement_pkey", &st)?;
    }

    let comments = get_comments(&hash_id, token)?;
    let mut comments = match comments.comments() {
        Some(comments) => comments.clone(),
        None => return Ok(()),
    };
    comments.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    for comment in comments {
        if let Some(source) = &comment.source {
            save::<model::source::Model, _>(&elephantry, "source_pkey", source)?;
        }

        if let Some(user) = &comment.user {
            save::<model::user::Model, _>(&elephantry, "user_pkey", user)?;
        }

        save::<model::comment::Model, _>(&elephantry, "comment_pkey", &comment)?;
    }

    Ok(())
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

    websocket(request, token)
}

fn get_statements(id: &str, token: &str) -> Result<data::Debates> {
    let request = format!(r#"["2","2","statements:video:{}","phx_join",{{}}]"#, id);

    websocket(request, token)
}

fn get_comments(id: &str, token: &str) -> Result<data::Debates> {
    let request = format!(r#"["3","3","comments:video:{}","phx_join",{{}}]"#, id);

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
