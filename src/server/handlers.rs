use crate::libs::gravatar;

pub async fn avatar_get(email_hash: String, params: gravatar::AvatarGetParams) -> Result<impl warp::Reply, warp::Rejection> {
    let base_url = "https://www.gravatar.com/avatar/";
    let query_string = gravatar::build_query_params(params);

    let mut url = format!("{}{}", base_url, email_hash);
    if !query_string.is_empty() {
        url.push_str("?");
        url.push_str(&query_string);
    }

    match gravatar::fetch_avatar(&url).await {
        Ok(bytes) => {
            let mut resp = warp::reply::Response::new(bytes.into());
            resp.headers_mut().insert("Content-Type", "image/jpeg".parse().unwrap());
            resp.headers_mut().insert("Cache-Control", "max-age=86400".parse().unwrap());
            resp.headers_mut().insert("Expires", "max-age=86400".parse().unwrap());
            resp.headers_mut().insert("Content-Disposition", format!("inline; filename=\"{}.jpg\"", email_hash).parse().unwrap());

            Ok(resp)
        }
        Err(err) => {
            log::error!("Error fetching avatar: {:?}", err);
            Err(err)
        }
    }
}
