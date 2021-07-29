mod data;
mod errors;
mod model;

use data::*;
use errors::*;
use std::convert::TryInto;
use structopt::StructOpt;

type Websocket = tungstenite::WebSocket<tungstenite::client::AutoStream>;

#[derive(StructOpt)]
struct Opt {
    video_hash_id: Option<String>,
    #[structopt(long)]
    limit: Option<u32>,
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    env_logger::init();

    let opt = Opt::from_args();

    let email = env("LOGIN_EMAIL")?;
    let password = env("LOGIN_PASSWORD")?;
    let token = login(&email, &password)?;

    let database_url = env("DATABASE_URL")?;
    let elephantry = elephantry::Pool::new(&database_url)?;

    let url = format!(
        "wss://api.captainfact.io/socket/websocket?token={}&vsn=2.0.0",
        token
    );

    let (mut websocket, _) = tungstenite::connect(&url)?;

    if let Some(video_hash_id) = opt.video_hash_id {
        save_video(&elephantry, &mut websocket, &video_hash_id)?;
    } else {
        let mut page = 1;

        loop {
            let data = get_summary(&token, page)?;
            let limit = opt.limit.unwrap_or_else(|| data.total_page());

            log::info!("Fetching page {}/{}", page, limit);

            for ref hash_id in data.hash_id() {
                if save_video(&elephantry, &mut websocket, hash_id).is_err() {
                    log::error!("Unable to save video '{}'", hash_id);
                }
            }

            if page == limit {
                break;
            } else {
                page += 1;
            }
        }
    }

    Ok(())
}

fn env(name: &str) -> Result<String> {
    std::env::var(name).map_err(|_| Error::Env(name.to_string()))
}

fn login(email: &str, password: &str) -> Result<String> {
    let query = serde_json::json!({
        "email": email,
        "password": password,
    });

    let response = attohttpc::post("https://api.captainfact.io/auth/identity/callback")
        .json(&query)?
        .send()?;
    let status = response.status();
    let json = response.json()?;

    if !status.is_success() {
        return Err(Error::Auth(status, json));
    }

    let json: serde_json::Value = json;
    if let Some(token) = json["token"].as_str() {
        Ok(token.to_string())
    } else {
        Err(Error::Auth(status, json))
    }
}

fn get_summary(token: &str, page: u32) -> Result<Data> {
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

fn save_video(
    elephantry: &elephantry::Connection,
    websocket: &mut Websocket,
    hash_id: &str,
) -> Result<()> {
    let debates = get_debates(websocket, hash_id)?;
    let video = match debates.video() {
        Some(video) => video,
        None => return Ok(()),
    };

    save::<model::video::Model, _>(elephantry, "video_pkey", video)?;

    for speaker in &video.speakers {
        save::<model::speaker::Model, _>(elephantry, "speaker_pkey", speaker)?;
    }

    let statements = get_statements(websocket, hash_id)?;
    let statements = match statements.statements() {
        Some(statements) => statements.clone(),
        None => Vec::new(),
    };

    for statement in statements {
        let mut st = statement.clone();
        st.video_id = video.id;

        save::<model::statement::Model, _>(elephantry, "statement_pkey", &st)?;
    }

    let comments = get_comments(websocket, hash_id)?;
    let mut comments = match comments.comments() {
        Some(comments) => comments.clone(),
        None => Vec::new(),
    };
    comments.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    for comment in comments {
        if let Some(source) = &comment.source {
            save::<model::source::Model, _>(elephantry, "source_pkey", source)?;
        }

        if let Some(user) = &comment.user {
            save::<model::user::Model, _>(elephantry, "user_pkey", user)?;
        }

        save::<model::comment::Model, _>(elephantry, "comment_pkey", &comment)?;
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

fn get_debates(websocket: &mut Websocket, id: &str) -> Result<data::Debates> {
    let request = format!(r#"["1","1","video_debate:{}","phx_join",{{}}]"#, id);

    get_data(websocket, request)
}

fn get_statements(websocket: &mut Websocket, id: &str) -> Result<data::Debates> {
    let request = format!(r#"["2","2","statements:video:{}","phx_join",{{}}]"#, id);

    get_data(websocket, request)
}

fn get_comments(websocket: &mut Websocket, id: &str) -> Result<data::Debates> {
    let request = format!(r#"["3","3","comments:video:{}","phx_join",{{}}]"#, id);

    get_data(websocket, request)
}

fn get_data(websocket: &mut Websocket, request: String) -> Result<data::Debates> {
    let mut max_tries = 10;
    websocket.write_message(tungstenite::Message::Text(request))?;

    loop {
        if max_tries < 0 {
            return Err(Error::WebsocketTryOut);
        }

        let response = websocket.read_message()?;

        match serde_json::from_str(response.to_text()?) {
            Ok(debates) => return Ok(debates),
            Err(_) => max_tries -= 1,
        }
    }
}
