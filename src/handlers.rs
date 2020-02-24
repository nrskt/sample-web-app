use warp::{Rejection, Reply};

use crate::{Database, User};

pub async fn list_users_handler(db: Database) -> Result<impl Reply, Rejection> {
    let db = db.lock().await;
    let users = db
        .clone()
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<User>>();
    Ok(warp::reply::json(&users))
}

pub async fn get_user_handler(db: Database, id: u64) -> Result<impl Reply, Rejection> {
    let db = db.lock().await;
    let user = db.get(&id);
    match user {
        None => Err(warp::reject::not_found()),
        Some(u) => Ok(warp::reply::json(&u)),
    }
}

pub async fn put_user_handler(db: Database, id: u64, user: User) -> Result<impl Reply, Rejection> {
    if id != user.id() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }
    let mut db = db.lock().await;
    db.insert(user.id(), user.clone());
    Ok(warp::reply::with_status(
        warp::reply::json(&user),
        warp::http::StatusCode::OK,
    ))
}
