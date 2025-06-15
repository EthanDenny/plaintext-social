use crate::entities::*;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};

pub async fn get_db_connection() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    Database::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

/* Users */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub display_name: String,
}

pub fn convert_user(user: user::Model) -> User {
    User {
        id: user.id,
        user_name: user.user_name,
        display_name: user.display_name,
    }
}

pub async fn get_user(user_id: i32) -> Option<User> {
    let db = get_db_connection().await;

    let user = user::Entity::find_by_id(user_id)
        .one(&db)
        .await
        .expect("Failed to get user");

    user.map(|user| convert_user(user))
}

pub async fn get_user_id_from_name(user_name: &str) -> Option<i32> {
    let db = get_db_connection().await;

    let user = user::Entity::find()
        .filter(user::Column::UserName.eq(user_name))
        .one(&db)
        .await
        .expect("Failed to get user by name");

    user.map(|user| user.id)
}

/* Posts */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    id: i32,
    author: Option<User>,
    content: String,
    timediff: String,
    replies: u64,
    likes: i32,
    parent_id: Option<i32>,
}

fn get_timediff(dt: chrono::DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now - dt;

    if diff.num_seconds() < 60 {
        "<1m".to_string()
    } else if diff.num_minutes() < 60 {
        format!("{}m", diff.num_minutes())
    } else if diff.num_hours() < 24 {
        format!("{}h", diff.num_hours())
    } else {
        dt.format("%Y-%m-%d").to_string()
    }
}

pub async fn get_replies(post_id: i32) -> Vec<Post> {
    let db = get_db_connection().await;

    let replies = post::Entity::find()
        .filter(post::Column::ParentId.eq(post_id))
        .all(&db)
        .await
        .expect("Failed to get replies");

    let mut replies_futures = replies.into_iter().map(convert_post);
    let replies: Vec<Post> = futures::future::join_all(&mut replies_futures).await;

    replies
}

async fn get_replies_count(post_id: i32) -> u64 {
    let db = get_db_connection().await;

    let replies_count = post::Entity::find()
        .filter(post::Column::ParentId.eq(post_id))
        .count(&db)
        .await
        .expect("Failed to count replies");

    replies_count
}

pub async fn convert_post(post: post::Model) -> Post {
    let author = get_user(post.user_id).await;
    let replies_count = get_replies_count(post.id).await;

    let timediff = get_timediff(post.created_at);

    Post {
        id: post.id,
        author,
        content: post.content,
        timediff,
        replies: replies_count,
        likes: post.likes,
        parent_id: post.parent_id,
    }
}

pub async fn create_post(user_id: i32, content: String, parent_id: Option<i32>) -> i32 {
    let db = get_db_connection().await;

    let new_post = post::ActiveModel {
        user_id: Set(user_id),
        content: Set(content),
        parent_id: Set(parent_id),
        ..Default::default()
    }
    .insert(&db)
    .await
    .expect("Failed to create post");

    new_post.id
}

pub async fn get_post(id: i32) -> Option<Post> {
    let db = get_db_connection().await;

    let post = post::Entity::find_by_id(id)
        .one(&db)
        .await
        .expect("Failed to get post");

    match post.map(|post| convert_post(post)) {
        Some(post) => Some(post.await),
        None => None,
    }
}

pub async fn get_posts() -> Vec<Post> {
    let db = get_db_connection().await;

    let posts: Vec<post::Model> = post::Entity::find()
        .all(&db)
        .await
        .expect("Failed to get posts");

    let mut posts_futures = posts.into_iter().map(convert_post);
    let posts: Vec<Post> = futures::future::join_all(&mut posts_futures).await;

    posts
}

pub async fn get_user_posts(user_id: i32, replies: bool) -> Vec<Post> {
    let db = get_db_connection().await;

    let posts: Vec<post::Model> = post::Entity::find()
        .filter(post::Column::UserId.eq(user_id))
        .filter(post::Column::ParentId.is_null().eq(!replies))
        .all(&db)
        .await
        .expect("Failed to get posts by user");

    let mut posts_futures = posts.into_iter().map(convert_post);
    let posts: Vec<Post> = futures::future::join_all(&mut posts_futures).await;

    posts
}
