use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod constants;
mod likes;
mod tweets;

// route index
async fn index() -> impl Responder {
    HttpResponse::Ok().body("v0.1.0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(tweets::get_tweets)
            .service(tweets::post_tweet)
            .service(tweets::get_tweet_by_id)
            .service(tweets::delete_tweet_by_id)
            .service(likes::get_likes_by_tweet_id)
            .service(likes::delete_likes_by_tweet_id)
            .service(likes::post_likes_by_tweet_id)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
