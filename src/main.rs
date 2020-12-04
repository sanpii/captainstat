mod data;
mod errors;

use data::*;
use errors::*;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let token = std::env::var("TOKEN").expect("Missing TOKEN env variable");

    let videos = videos(&token)?;

    dbg!(videos);

    Ok(())
}

fn videos(token: &str) -> Result<Vec<Video>> {
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
