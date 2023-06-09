use std::env;

use crate::apps::*;
use actix_files::Files;
use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{
    cookie::{self, Key},
    error, get, http,
    middleware::Logger,
    web, App, Error, HttpResponse, HttpServer, Responder, Result,
};
use dotenv::dotenv;
use serde::*;
use sqlx::{sqlite::{SqliteConnectOptions, SqliteJournalMode}, ConnectOptions, SqlitePool};
use std::str::FromStr;
use tera::{Context, Tera};
use validator;

mod apps;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db = env::var("DATABASE_URL").expect("Database_url not found in env");
    let conn = sqlx::SqlitePool::connect(&db).await.unwrap();
    

    HttpServer::new(move || {

        let mut templates = Tera::new("templates/**/*").expect("errors in tera templates");
        templates.autoescape_on(vec!["tera"]);

        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    // // customize session and cookie expiration
                    // .session_lifecycle(
                    //     PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                    // )
                    .build(),
            )
            .app_data(web::Data::new(templates))
            .app_data(web::Data::new(conn.clone()))
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/login")
                    .route(web::get().to(login))
                    .route(web::post().to(post_login)),
            )
            .service(
                web::resource("signin")
                    .route(web::get().to(signin))
                    .route(web::post().to(post_signin)),
            )
            .service(web::resource("logout").route(web::get().to(logout)))
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
