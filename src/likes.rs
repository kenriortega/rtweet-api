use diesel::r2d2::PooledConnection;
use std::str::FromStr;

use super::schema::likes;
use crate::constants;
use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::query_dsl::methods::FilterDsl;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[table_name = "likes"]
#[derive(Queryable, Insertable, Serialize, Deserialize)]
struct Like {
    id: Uuid,
    created_at: NaiveDateTime,
    tweet_id: Uuid,
}

impl Like {
    fn new(tweet_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            tweet_id,
        }
    }
}

// /tweets/:id/likes GET: get like by tweet by id
#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet_id(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let t_id = &path.0 .0; // tweet id desde los parametros de la url
    let t_id_uuid = Uuid::from_str(t_id); // tweet id formateado a uuid

    if t_id_uuid.is_err() {
        println!("tweet id inválido, error: {:?}", t_id_uuid.err());
        // si no pudimos convertir a un uuid válido, asumimos que el tweet no existe.
        return HttpResponse::NotFound().await.unwrap();
    }

    let conn = pool
        .get()
        .expect("No pude obtener conexión a la base de datos");
    let response = list_likes(&conn, t_id_uuid.unwrap());

    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(response)
}

fn list_likes(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    t_id_uuid: Uuid,
) -> Vec<Like> {
    use crate::schema::likes::dsl::*;

    let result = likes.filter(tweet_id.eq(t_id_uuid)).load::<Like>(conn);

    match result {
        Ok(rows) => rows,
        Err(_) => vec![],
    }
}
// /tweets/:id/likes DELETE: delete like by tweet by id
#[delete("/tweets/{id}/likes")]
pub async fn delete_likes_by_tweet_id(
    path: Path<(String,)>,
    _pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let tweet = format!("tweet: {:?}", path.0);
    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(tweet)
}
// /tweets/:id/likes POST: create like by tweet by id
#[post("/tweets/{id}/likes")]
pub async fn post_likes_by_tweet_id(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    use crate::schema::likes::dsl::*;
    let t_id = &path.0 .0;
    let conn = pool.get().expect("Error to insert");
    let like = Like::new(Uuid::from_str(t_id).unwrap());
    let result = diesel::insert_into(likes)
        .values(like)
        .execute(&conn)
        .unwrap();
    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(result)
}
