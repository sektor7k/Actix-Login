use actix_files::Files;
use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Responder, Result, middleware::Logger, error, http, cookie::{Key, self}};
use serde::*;
use tera::{Tera,Context};
use dotenv::dotenv;
use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware,
};

#[derive(Debug, Deserialize)]
pub struct LoginUser{
    email: String,
    password: String
}


pub async fn index(tmpl:web::Data<Tera> , session: Session) -> Result<HttpResponse,Error> {

    let mut ctx = Context::new();

    if let Some(user) = session.get::<String>("user")?{
      ctx.insert("user", &user)  
    }
    
    let a = tmpl.render("index.html", &ctx)
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}



pub async fn login(tmpl:web::Data<Tera> ,session:Session) -> Result<HttpResponse,Error> {
    
    
    if let Some(user) = session.get::<String>("user")?{
        return Ok(redirct("/"));
    }
    let mut ctx = Context::new();
    let a = tmpl.render("login.html", &ctx)
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}
pub async fn logout(session:Session) -> Result<HttpResponse,Error> {
    
    session.purge();
    
    return Ok(redirct("/"));
}


pub async fn post_login(tmpl:web::Data<Tera>, form: web::Form<LoginUser>, session: Session) -> Result<HttpResponse,Error> {
    let ctx = Context::new();
    
    session.insert("user", &form.email)?;

    let a = tmpl.render("login.html", &ctx)
    .map_err(error::ErrorInternalServerError)?;
    Ok(redirct("/"))
        
}

pub fn redirct(location:&str)-> HttpResponse{
    HttpResponse::Found()
        .append_header((http::header::LOCATION, location))
        .finish()
}