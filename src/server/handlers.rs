use reqwest::Client;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AvatarGetParams {
    s: Option<u32>,
    d: Option<String>,
    f: Option<bool>,
    r: Option<String>,
}

pub async fn avatar_get(email_hash: String, params: AvatarGetParams) -> Result<impl warp::Reply, warp::Rejection> {
    let base_url = "https://www.gravatar.com/avatar/";

    let mut url = format!("{}{}", base_url, email_hash);
    let mut query_params = vec![];

    params.s.map(|size| { query_params.push(format!("s={}", size)) });
    params.d.map(|default| { query_params.push(format!("d={}", default)) });
    params.f.map(|force| if force { query_params.push(String::from("f=y")) });
    params.r.map(|rating| { query_params.push(format!("r={}", rating)) });

    if !query_params.is_empty() {
        url.push('?');
        url += &query_params.join("&");
    }

    match Client::new().get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes().await {
                    Ok(bytes) => Ok(warp::reply::Response::new(bytes.into())),
                    Err(_) => Err(warp::reject::not_found()),
                }
            } else {
                log::error!("Gravatar returned status code: {}", response.status());
                Err(warp::reject::not_found())
            }
        }
        Err(err) => {
            log::error!("Error fetching avatar: {}", err);
            Err(warp::reject::not_found())
        }
    }
}
