mod db;
mod entities;
mod routes;

use actix_files::Files;
use actix_web::{middleware::NormalizePath, App, HttpServer};
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .service(Files::new("/static", "./static").show_files_listing())
            .service(index)
            .service(feed)
            .service(post_page)
            .service(user_page_default)
            .service(user_page_replies)
            .service(new_post)
            .service(new_reply)
            .service(login)
            .service(new_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use sea_orm::{ActiveValue::Set, EntityTrait};

    use crate::entities::{post, user};

    use super::*;

    async fn create_user_and_posts(user_name: &str, display_name: &str, posts: Vec<&str>) {
        let conn = db::get_db_connection().await;

        let result = user::Entity::insert(user::ActiveModel {
            display_name: Set(display_name.to_string()),
            user_name: Set(user_name.to_string()),
            ..Default::default()
        })
        .exec(&conn)
        .await
        .expect("Failed to create user");

        for post_content in posts {
            post::Entity::insert(post::ActiveModel {
                content: Set(post_content.to_string()),
                user_id: Set(result.last_insert_id),
                ..Default::default()
            })
            .exec(&conn)
            .await
            .expect("Failed to create post");
        }
    }

    #[actix_rt::test]
    async fn create_data() {
        dotenvy::dotenv().expect("Failed to load .env file");

        create_user_and_posts(
            "ethandenny",
            "Ethan Denny",
            vec!["just setting up my plaintext.social"],
        )
        .await;

        create_user_and_posts(
            "johndoe",
            "John Doe",
            vec!["Hello, world!", "My second post!"],
        )
        .await;

        create_user_and_posts("janedoe", "Jane Doe", vec!["This is Jane's first post."]).await;
        create_user_and_posts("alice", "Alice Smith", vec!["Alice's first post."]).await;
        create_user_and_posts("bob", "Bob Johnson", vec!["Bob's first post."]).await;
        create_user_and_posts("charlie", "Charlie Brown", vec!["Charlie says hi!"]).await;
        create_user_and_posts("dave", "Dave White", vec!["Dave's first post."]).await;
    }
}
