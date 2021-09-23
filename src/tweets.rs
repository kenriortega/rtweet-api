use crate::constants;
use actix_web::web::Path;
use actix_web::{delete, get, post, HttpResponse};
// /tweet GET: all tweets
#[get("/tweets")]
pub async fn get_tweets() -> HttpResponse {
    let tweets = ["tweet 1: hola", "tweet 2: chao"];
    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(tweets)
}
// /tweet POST: create tweet
#[post("/tweets")]
pub async fn post_tweet() -> HttpResponse {
    let new_tweet = "tweet 1: hola";
    HttpResponse::Created()
        .content_type(constants::APPLICATION_JSON)
        .json(new_tweet)
}
// /tweets/:id GET: get tweet by id
#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("tweet: {:?}", path.0);
    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(tweet)
}

// /tweets/:id DELETE: delete tweet by id
#[delete("/tweets/{id}")]
pub async fn delete_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("tweet: {:?}", path.0);
    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(tweet)
}
