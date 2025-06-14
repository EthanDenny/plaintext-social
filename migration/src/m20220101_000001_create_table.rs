use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    UserId,
    Content,
    CreatedAt,
    Replies,
    Likes,
    ParentId,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    UserName,
    DisplayName,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::UserName))
                    .col(string(User::DisplayName))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(integer(Post::UserId))
                    .col(string(Post::Content))
                    .col(timestamp(Post::CreatedAt).default(Expr::cust("CURRENT_TIMESTAMP")))
                    .col(integer(Post::Replies).default(0))
                    .col(integer(Post::Likes).default(0))
                    .col(integer(Post::ParentId).null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Post::Table, Post::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Post::Table, Post::ParentId)
                            .to(Post::Table, Post::Id)
                            .on_delete(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;

        Ok(())
    }
}
