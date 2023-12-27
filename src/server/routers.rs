use warp::Filter;

use crate::libs::gravatar;

use super::handlers;

pub fn router() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    avatar_get()
}

fn avatar_get() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    warp::path!("avatar" / String)
        .and(warp::get())
        .and(warp::query::<gravatar::AvatarGetParams>())
        .and_then(handlers::avatar_get)
}
