use crate::constants;
use actix_web::web::Path;
use actix_web::{delete, get, post, HttpResponse};

// /tweets/:id/likes GET: get like by tweet by id
#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("tweet: {:?}", path.0);
    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(tweet)
}

// /tweets/:id/likes DELETE: delete like by tweet by id
#[delete("/tweets/{id}/likes")]
pub async fn delete_likes_by_tweet_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("tweet: {:?}", path.0);
    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(tweet)
}
// /tweets/:id/likes POST: create like by tweet by id
#[post("/tweets/{id}/likes")]
pub async fn post_likes_by_tweet_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("tweet: {:?}", path.0);
    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(tweet)
}
