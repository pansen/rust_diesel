use actix::*;
use actix_web::{
    AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Path,
};
use futures::Future;

use super::super::db::{CreateUser, DbExecutor};

/// State with DbExecutor address
pub struct AppState {
    pub db: Addr<DbExecutor>,
}

/// Async request handler
pub fn index((name, req): (Path<String>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    info!("adding a new user named: {} ...", name);

    // send async `CreateUser` message to a `DbExecutor`
    req.state()
        .db
        .send(CreateUser {
            name: name.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => {
                info!("that new user: {} was created with id: {}", user.name, user.id);
                Ok(HttpResponse::Ok().json(user))
            }
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
