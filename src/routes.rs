use crate::db;

use actix_web::{get, post, web, HttpResponse, Responder};
use lazy_static::lazy_static;

use serde::Deserialize;
use tera::{Context, Error as TeraError, Tera};

const BASE: &str = "http://127.0.0.1:8080";

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

fn render_error(err: TeraError) -> HttpResponse {
    eprintln!("Template error: {:#?}", err);
    HttpResponse::InternalServerError().body("Internal Server Error")
}

#[get("/")]
async fn index() -> impl Responder {
    let posts = db::get_posts().await;

    let mut context = Context::new();
    context.insert("base", BASE);
    context.insert("posts", &posts);

    match TEMPLATES.render("index.html", &context) {
        Ok(body) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body),
        Err(err) => render_error(err),
    }
}

#[get("/post/{id}")]
async fn post_page(id: web::Path<usize>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h1>Post: {id}</h1>"))
}

async fn user_page_base(user: web::Path<String>, replies: bool) -> impl Responder {
    let user_id = db::get_user_id_from_name(user.as_str()).await;
    let user = match user_id {
        Some(id) => db::get_user(id).await,
        None => None,
    };

    match user {
        Some(user) => {
            let posts = db::get_user_posts(user.id, replies).await;

            let mut context = Context::new();
            context.insert("base", BASE);
            context.insert("replies", &replies);
            context.insert("user_name", user.user_name.as_str());
            context.insert("display_name", user.display_name.as_str());
            context.insert("posts", &posts);

            match TEMPLATES.render("user.html", &context) {
                Ok(body) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(body),
                Err(err) => render_error(err),
            }
        }
        None => {
            return HttpResponse::NotFound().body("User not found");
        }
    }
}

#[get("/user/{user}")]
async fn user_page_default(user: web::Path<String>) -> impl Responder {
    user_page_base(user, false).await
}

#[get("/user/{user}/replies")]
async fn user_page_replies(user: web::Path<String>) -> impl Responder {
    user_page_base(user, true).await
}

#[derive(Deserialize)]
struct NewPost {
    content: String,
}

#[post("/post")]
async fn new_post(post: web::Json<NewPost>) -> impl Responder {
    println!("New post: {}", post.content);
    HttpResponse::Created().finish()
}
