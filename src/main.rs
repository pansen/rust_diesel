//! Actix web diesel example
//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate r2d2;
extern crate uuid;

#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

use dotenv::dotenv;

use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path,
    State,
};

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use futures::Future;

mod db;
mod models;
mod schema;

use db::{CreateUser, DbExecutor};

/// State with DbExecutor address
struct AppState {
    db: Addr<DbExecutor>,
}

/// Async request handler
fn index(
    (name, state): (Path<String>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    state
        .db
        .send(CreateUser {
            name: name.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    dotenv().ok();
    // this modules name is `rust_diesel`, inherited from the project name `rust-diesel`
    ::std::env::set_var("RUST_LOG", "rust_diesel=info,actix_web=info");
    env_logger::init();

    info!("logging initialized");
    let sys = actix::System::new("rust_diesel");

    info!("starting with DB name: {}", dotenv!("DB_NAME"));
    // Start 3 db executor actors
    let manager =
        ConnectionManager::<SqliteConnection>::new(dotenv!("DB_NAME"));
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3,
                                  move || DbExecutor(pool.clone()));

    // Start http server
    server::new(move || {
        App::with_state(AppState { db: addr.clone() })
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/{name}", |r|
                r.method(http::Method::GET)
                    .with(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    info!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
