use super::schema::tweets;
use crate::constants;
use crate::diesel::QueryDsl;
use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[table_name = "tweets"]
#[derive(Queryable, Insertable, Serialize, Deserialize)]
struct Tweet {
    id: Uuid,
    created_at: NaiveDateTime,
    message: String,
}

impl Tweet {
    fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message,
        }
    }
}

// /tweet GET: all tweets
#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    use crate::schema::tweets::dsl::*;
    let conn = pool.get().expect("Error to insert");
    let result = tweets
        .order(created_at.desc())
        .limit(3)
        .load::<Tweet>(&conn);

    let response = match result {
        Ok(tws) => tws,
        Err(_) => vec![],
    };
    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(response)
}
// /tweet POST: create tweet
#[post("/tweets")]
pub async fn post_tweet(
    req_body: String,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let new_tweet = Tweet::new(req_body);
    let conn = pool.get().expect("Error to insert");
    diesel::insert_into(tweets::table)
        .values(&new_tweet)
        .execute(&conn)
        .expect("error on exceute insert");
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
