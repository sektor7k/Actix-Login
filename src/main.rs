use actix_files::Files;
use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Responder, Result, middleware::Logger, error, http, cookie::{Key, self}};
use crate::apps::{*};
use serde::*;
use tera::{Tera,Context};
use dotenv::dotenv;
use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware,
};

mod apps;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
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
                 .build()
        )
        .app_data(web::Data::new(templates))
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/login")
                .route(web::get().to(login))
                .route(web::post().to(post_login)))
        .service(web::resource("signin").route(web::get().to(signin)))
        .service(web::resource("logout").route(web::get().to(logout)))
        .service(Files::new("/static", "static").show_files_listing())
        
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
