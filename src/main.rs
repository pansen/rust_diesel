//! Actix web diesel example
//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.

extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use actix_web::{
    App, http, middleware, server,
};
use dotenv::dotenv;
use actix::*;
use self::db::DbExecutor;

mod db;
mod models;
mod schema;
mod handlers;

use self::handlers::index::{
    ws_index_raw,
};


/// State with DbExecutor address
pub struct DieselAppState {
    pub db: Addr<DbExecutor>,
}


fn main() {
    dotenv().ok();
    // this modules name is `rust_diesel`, inherited from the project name `rust-diesel`
    ::std::env::set_var("RUST_LOG", "rust_diesel=info,actix_web=info");
    env_logger::init();

    info!("logging initialized");
    let sys = actix::System::new("rust_diesel");

    // Start http server
    server::new(move || {
        App::with_state(DieselAppState {
            db: db::db_executor().clone()
        })
            // enable logger
            .middleware(middleware::Logger::default())
            // websocket route
            .resource(
                "/ws/",
                |r| r.method(http::Method::GET).f(ws_index_raw),
            )
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    info!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}


#[cfg(test)]
mod tests {
    extern crate env_logger;

    use actix_web::test::TestServer;

    use super::*;

    use self::http::Method;

    #[test]
    fn env_is_configured() {
        ::std::env::set_var("RUST_LOG", "rust_diesel=info,actix_web=info");
        let _ = env_logger::try_init();
        info!("can log from the test too");
        assert_ne!("", dotenv!("DATABASE_URL"))
    }

    #[test]
    fn will_respond_200() {
        ::std::env::set_var("RUST_LOG", "rust_diesel=info,actix_web=info");
        let _ = env_logger::try_init();

        // Create a testserver
        // https://github.com/actix/actix-website/blob/master/content/docs/testing.md
        let mut srv = TestServer::build_with_state(|| {
            // then we can construct custom state, or it could be `()`
            DieselAppState { db: db::db_executor() }
        })
            // register server handlers and start test server
            .start(|app| {
                app.resource("/{name}", |r|
                    r.method(http::Method::GET)
                        .with(handlers::index::index));
            });

        let path = "/andi";
        let request = srv.client(Method::GET, path).finish().unwrap();
        let response = srv.execute(request.send()).unwrap();
        info!("response from: {} is {:?}", path, response.status());
        assert!(response.status().is_success());

        let path = "/";
        let request = srv.client(Method::GET, path).finish().unwrap();
        let response = srv.execute(request.send()).unwrap();
        info!("response from: {} is {:?}", path, response.status());
        assert_eq!(404, response.status());
    }
}