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
    let context = Context::new();

    match TEMPLATES.render("index.html", &context) {
        Ok(body) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body),
        Err(err) => render_error(err),
    }
}

#[get("/feed")]
async fn feed() -> impl Responder {
    let posts = db::get_posts().await;

    let mut context = Context::new();
    context.insert("base", BASE);
    context.insert("posts", &posts);

    match TEMPLATES.render("feed.html", &context) {
        Ok(body) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body),
        Err(err) => render_error(err),
    }
}

#[get("/post/{id}")]
async fn post_page(id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();

    let post = db::get_post(id).await;
    let replies = db::get_replies(id).await;

    let mut context = Context::new();
    context.insert("base", BASE);
    context.insert("post", &post);
    context.insert("replies", &replies);

    match TEMPLATES.render("post.html", &context) {
        Ok(body) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body),
        Err(err) => render_error(err),
    }
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
    user_name: String,
    content: String,
}

#[post("/post")]
async fn new_post(post: web::Json<NewPost>) -> impl Responder {
    let user_id = db::get_user_id_from_name(&post.user_name).await;
    if let Some(user_id) = user_id {
        db::create_post(user_id, post.into_inner().content, None).await;
        HttpResponse::Created().finish()
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[derive(Deserialize)]
struct NewReply {
    user_name: String,
    content: String,
    parent_id: i32,
}

#[post("/reply")]
async fn new_reply(post: web::Json<NewReply>) -> impl Responder {
    let user_id = db::get_user_id_from_name(&post.user_name).await;
    if let Some(user_id) = user_id {
        db::create_post(user_id, post.content.clone(), Some(post.parent_id)).await;
        HttpResponse::Created().finish()
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[derive(Deserialize)]
struct Login {
    user_name: String,
}

#[post("/login")]
async fn login(data: web::Json<Login>) -> impl Responder {
    let user_exists = db::user_name_exists(&data.user_name).await;

    println!(
        "Tried to login with user: {}, user exist: {}",
        data.user_name, user_exists
    );

    HttpResponse::Created()
        .content_type("application/json")
        .body(format!(r#"{{"new_account": {}}}"#, !user_exists))
}

#[derive(Deserialize)]
struct NewUser {
    user_name: String,
    display_name: String,
}

#[post("/user/new")]
async fn new_user(data: web::Json<NewUser>) -> impl Responder {
    let success = db::create_user(&data.user_name, &data.display_name).await;

    if success {
        HttpResponse::Created().finish()
    } else {
        HttpResponse::BadRequest().finish()
    }
}
