use serde_derive::Deserialize;
use warp::reject::Rejection;

#[derive(Debug, Deserialize)]
pub struct AvatarGetParams {
    s: Option<u32>,
    d: Option<String>,
    f: Option<bool>,
    r: Option<String>,
}

pub fn build_query_params(params: AvatarGetParams) -> String {
    let mut query_params = vec![];

    params.s.map(|size| query_params.push(format!("s={}", size)));
    params.d.map(|default| query_params.push(format!("d={}", default)));
    params.f.map(|force| if force { query_params.push(String::from("f=y")) });
    params.r.map(|rating| query_params.push(format!("r={}", rating)));

    query_params.join("&")
}

pub async fn fetch_avatar(url: &str) -> Result<Vec<u8>, Rejection> {
    let response = reqwest::get(url).await.map_err(|err| {
        log::error!("Error fetching avatar: {}", err);
        warp::reject::not_found()
    })?;

    if !response.status().is_success() {
        log::error!("Gravatar returned status code: {}", response.status());
        return Err(warp::reject::not_found());
    }

    match response.bytes().await {
        Ok(bytes) => {
            Ok(bytes.to_vec())
        }
        Err(err) => {
            log::error!("Error reading response bytes: {}", err);
            Err(warp::reject::not_found())
        }
    }
}
