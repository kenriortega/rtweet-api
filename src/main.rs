#[macro_use]
extern crate diesel;
mod constants;
mod likes;
mod schema;
mod tweets;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;
// route index
async fn index() -> impl Responder {
    HttpResponse::Ok().body("v0.1.0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Not found");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Can`t create a pool");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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
