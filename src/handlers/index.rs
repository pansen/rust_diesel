use actix::*;
use actix_web::{
    AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Path, Error, ws,
};
use futures::Future;
use super::ws::DieselWebSocket;

use super::super::DieselAppState;
use super::super::db::{CreateUser, };


/// do websocket handshake and start `MyWebSocket` actor
/// see:
/// - https://actix.rs/docs/extractors/
pub fn ws_index_raw(r: &HttpRequest<DieselAppState>) -> Result<HttpResponse, Error> {
    // let params = Path::<(String, String)>::extract(r);

    // start<A, S>(req: &HttpRequest<S>, actor: A) -> Result<HttpResponse, Error>
    let diesel_web_socket = DieselWebSocket::new();
    ws::start(r, diesel_web_socket)
}

/// Async request handler
pub fn index((name, req): (Path<String>, HttpRequest<DieselAppState>)) -> FutureResponse<HttpResponse> {
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
