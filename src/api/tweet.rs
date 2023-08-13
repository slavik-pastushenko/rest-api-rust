use actix_web::web;
use actix_web::{ web::{ Data, Json }, post, put, get, delete, HttpResponse };
use crate::{ models::tweet::Tweet, repository::tweet::Database };

#[get("/tweets")]
pub async fn get_tweets(db: web::Data<Database>) -> HttpResponse {
    let tweets = db.get_tweets();

    HttpResponse::Ok().json(tweets)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let tweet = db.get_tweet_by_id(&id);

    match tweet {
        Some(tweet) => HttpResponse::Ok().json(tweet),
        None => HttpResponse::NotFound().body("Tweet was not found"),
    }
}

#[post("/tweets")]
pub async fn create_tweet(db: Data<Database>, payload: Json<Tweet>) -> HttpResponse {
    let tweet = db.create_tweet(payload.into_inner());

    match tweet {
        Ok(tweet) => HttpResponse::Ok().json(tweet),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/tweets/{id}")]
pub async fn update_tweet(
    db: web::Data<Database>,
    id: web::Path<String>,
    payload: web::Json<Tweet>
) -> HttpResponse {
    let tweet = db.update_tweet(&id, payload.into_inner());

    match tweet {
        Some(tweet) => HttpResponse::Ok().json(tweet),
        None => HttpResponse::NotFound().body("Tweet was not found"),
    }
}

#[delete("/tweets/{id}")]
pub async fn delete_tweet(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let tweet = db.delete_tweet(&id);

    match tweet {
        Some(tweet) => HttpResponse::Ok().json(tweet),
        None => HttpResponse::NotFound().body("Tweet was not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/api")
            .service(get_tweets)
            .service(get_tweet_by_id)
            .service(create_tweet)
            .service(update_tweet)
            .service(delete_tweet)
    );
}
