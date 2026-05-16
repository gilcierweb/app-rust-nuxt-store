use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_users_pid")
                    .table(Users::Table)
                    .col(Users::Pid)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_orders_created_at")
                    .table(Orders::Table)
                    .col(Orders::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_orders_user_id")
                    .table(Orders::Table)
                    .col(Orders::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_order_items_order_id")
                    .table(OrderItems::Table)
                    .col(OrderItems::OrderId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_order_items_product_id")
                    .table(OrderItems::Table)
                    .col(OrderItems::ProductId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_product_images_product_id")
                    .table(ProductImages::Table)
                    .col(ProductImages::ProductId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_product_images_product_id")
                    .table(ProductImages::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_order_items_product_id")
                    .table(OrderItems::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_order_items_order_id")
                    .table(OrderItems::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_orders_user_id")
                    .table(Orders::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_orders_created_at")
                    .table(Orders::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_users_pid")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Pid,
}

#[derive(Iden)]
enum Orders {
    Table,
    CreatedAt,
    UserId,
}

#[derive(Iden)]
enum OrderItems {
    Table,
    OrderId,
    ProductId,
}

#[derive(Iden)]
enum ProductImages {
    Table,
    ProductId,
}
