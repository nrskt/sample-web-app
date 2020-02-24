use warp::{Filter, Rejection, Reply};

use crate::{get_user_handler, list_users_handler, put_user_handler, Database};

pub fn users_api(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_user(db.clone()).or(list(db.clone())).or(put_user(db))
}

fn users() -> warp::filters::BoxedFilter<()> {
    warp::path("users").boxed()
}

fn user_id() -> warp::filters::BoxedFilter<(u64,)> {
    warp::path::param().boxed()
}

fn list(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    users()
        .and(warp::get())
        .and_then(move || list_users_handler(db.clone()))
}

fn get_user(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    users()
        .and(user_id())
        .and(warp::get())
        .and_then(move |id| get_user_handler(db.clone(), id))
}

fn put_user(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    users()
        .and(user_id())
        .and(warp::put())
        .and(warp::body::json())
        .and_then(move |id, body| put_user_handler(db.clone(), id, body))
}
