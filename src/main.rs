use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

const BASE: &str = "http://127.0.0.1:8080";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    id: usize,
    display_name: &'static str,
    account: &'static str,
    content: &'static str,
    created_at: String,
    replies: usize,
    likes: usize,
    parent: Option<usize>,
}

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
fn format_datetime(dt: &str) -> String {
    let then = DateTime::parse_from_rfc3339(dt)
        .expect("Failed to parse datetime")
        .with_timezone(&Utc);
    let now = Utc::now();
    let diff = now - then;

    if diff.num_seconds() < 60 {
        "< 1m".to_string()
    } else if diff.num_minutes() < 60 {
        format!("{}m", diff.num_minutes())
    } else if diff.num_hours() < 24 {
        format!("{}h", diff.num_hours())
    } else {
        then.format("%Y-%m-%d").to_string()
    }
}

fn get_display_name(account: &str) -> &'static str {
    match account {
        "ethandenny" => "Ethan Denny",
        "janedoe" => "Jane Doe",
        _ => "Unknown User",
    }
}

fn message(
    id: usize,
    account: &'static str,
    content: &'static str,
    created_at: &str,
    replies: usize,
    likes: usize,
    parent: Option<usize>,
) -> Message {
    Message {
        id,
        display_name: get_display_name(account),
        account,
        content,
        created_at: String::from(created_at),
        replies,
        likes,
        parent,
    }
}

fn resolve_created_at(message: &Message) -> Message {
    let created_at = format_datetime(&message.created_at);
    Message {
        id: message.id,
        display_name: message.display_name,
        account: message.account,
        content: message.content,
        created_at,
        replies: message.replies,
        likes: message.likes,
        parent: message.parent,
    }
}

lazy_static! {
    static ref MESSAGES: Vec<Message> = vec![
        message(
            0,
            "ethandenny",
            "just setting up my txt.social",
            "2025-06-13T12:00:00Z",
            0,
            0,
            None
        ),
        message(
            1,
            "janedoe",
            "Hello, world!",
            "2023-10-01T12:05:00Z",
            0,
            0,
            None
        ),
    ];
}

#[get("/")]
async fn index() -> impl Responder {
    let messages = &*(MESSAGES
        .iter()
        .map(|message| resolve_created_at(message))
        .collect::<Vec<_>>());

    let mut context = Context::new();
    context.insert("base", BASE);
    context.insert("messages", messages);

    match TEMPLATES.render("index.html", &context) {
        Ok(body) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body),
        Err(err) => {
            eprintln!("Template error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

#[get("/message/{id}")]
async fn message_page(id: web::Path<usize>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h1>Message: {id}</h1>"))
}

fn user_page_base(user: web::Path<String>, messages: &[Message], replies: bool) -> impl Responder {
    let mut context = Context::new();
    context.insert("base", BASE);
    context.insert("replies", &replies);
    context.insert("name", get_display_name(user.as_str()));
    context.insert("account", user.as_str());
    context.insert("messages", messages);

    match TEMPLATES.render("user.html", &context) {
        Ok(body) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body),
        Err(err) => {
            eprintln!("Template error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

#[get("/user/{user}")]
async fn user_page_default(user: web::Path<String>) -> impl Responder {
    let messages = &*(MESSAGES
        .iter()
        .filter(|message| message.account == user.as_str())
        .map(|message| resolve_created_at(message))
        .collect::<Vec<_>>());

    user_page_base(user, messages, false)
}

#[get("/user/{user}/replies")]
async fn user_page_replies(user: web::Path<String>) -> impl Responder {
    let messages = &*(MESSAGES
        .iter()
        .filter(|message| message.account == user.as_str() && message.parent.is_some())
        .map(|message| resolve_created_at(message))
        .collect::<Vec<_>>());

    user_page_base(user, messages, true)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(Files::new("/static", "./static").show_files_listing())
            .service(index)
            .service(message_page)
            .service(user_page_default)
            .service(user_page_replies)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
