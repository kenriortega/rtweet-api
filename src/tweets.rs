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
use std::str::FromStr;
use uuid::Uuid;
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "tweets"]
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
    let conn = pool.get().expect("Can`t connect to the DB");
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
    let conn = pool.get().expect("Can`t connect to the DB");
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
pub async fn get_tweet_by_id(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    use crate::schema::tweets::dsl::*;
    let t_id = &path.0 .0; // tweet id desde los parametros de la url
    let t_id_uuid = Uuid::from_str(t_id); // tweet id formateado a uuid

    if t_id_uuid.is_err() {
        println!("invalid tweet id, error: {:?}", t_id_uuid.err());
        // si no pudimos convertir a un uuid válido, asumimos que el tweet no existe.
        return HttpResponse::NotFound().await.unwrap();
    }

    let conn = pool.get().expect("Can`t connect to the DB");

    let result = tweets.find(t_id_uuid.unwrap()).first::<Tweet>(&conn);
    let tweet = result.expect("Not found");

    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(tweet)
}

// /tweets/:id DELETE: delete tweet by id
#[delete("/tweets/{id}")]
pub async fn delete_tweet_by_id(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    use crate::schema::tweets::dsl::*;
    let t_id = &path.0 .0; // tweet id desde los parametros de la url
    let t_id_uuid = Uuid::from_str(t_id); // tweet id formateado a uuid

    if t_id_uuid.is_err() {
        println!("invalid tweet id, error: {:?}", t_id_uuid.err());
        // si no pudimos convertir a un uuid válido, asumimos que el tweet no existe.
        return HttpResponse::NotFound().await.unwrap();
    }
    let conn = pool.get().expect("Can`t connect to the DB");

    let num_deleted = diesel::delete(tweets.filter(id.eq(t_id_uuid.unwrap())))
        .execute(&conn)
        .expect("Error deleting tweet");

    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(num_deleted)
}
