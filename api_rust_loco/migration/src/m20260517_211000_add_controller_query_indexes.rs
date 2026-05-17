use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_coupons_code")
                    .table(Coupons::Table)
                    .col(Coupons::Code)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payments_order_id")
                    .table(Payments::Table)
                    .col(Payments::OrderId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_addresses_user_id_created_at")
                    .table(Addresses::Table)
                    .col(Addresses::UserId)
                    .col(Addresses::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_shipments_order_id")
                    .table(Shipments::Table)
                    .col(Shipments::OrderId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_shipments_created_at")
                    .table(Shipments::Table)
                    .col(Shipments::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_posts_created_at")
                    .table(Posts::Table)
                    .col(Posts::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_created_at")
                    .table(Users::Table)
                    .col(Users::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_products_category_id")
                    .table(Products::Table)
                    .col(Products::CategoryId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_banner_positions_code_is_active")
                    .table(BannerPositions::Table)
                    .col(BannerPositions::Code)
                    .col(BannerPositions::IsActive)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_banners_active_lookup")
                    .table(Banners::Table)
                    .col(Banners::PositionId)
                    .col(Banners::Status)
                    .col(Banners::Device)
                    .col(Banners::StartAt)
                    .col(Banners::EndAt)
                    .col(Banners::Priority)
                    .col(Banners::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_banner_events_banner_created_type")
                    .table(BannerEvents::Table)
                    .col(BannerEvents::BannerId)
                    .col(BannerEvents::CreatedAt)
                    .col(BannerEvents::EventType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_banner_events_created_at")
                    .table(BannerEvents::Table)
                    .col(BannerEvents::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_banner_events_created_at")
                    .table(BannerEvents::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_banner_events_banner_created_type")
                    .table(BannerEvents::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_banners_active_lookup")
                    .table(Banners::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_banner_positions_code_is_active")
                    .table(BannerPositions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_products_category_id")
                    .table(Products::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_users_created_at")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_posts_created_at")
                    .table(Posts::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_shipments_created_at")
                    .table(Shipments::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_shipments_order_id")
                    .table(Shipments::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_addresses_user_id_created_at")
                    .table(Addresses::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_payments_order_id")
                    .table(Payments::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_coupons_code")
                    .table(Coupons::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Coupons {
    Table,
    Code,
}

#[derive(Iden)]
enum Payments {
    Table,
    OrderId,
}

#[derive(Iden)]
enum Addresses {
    Table,
    UserId,
    CreatedAt,
}

#[derive(Iden)]
enum Shipments {
    Table,
    OrderId,
    CreatedAt,
}

#[derive(Iden)]
enum Posts {
    Table,
    CreatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    CreatedAt,
}

#[derive(Iden)]
enum Products {
    Table,
    CategoryId,
}

#[derive(Iden)]
enum BannerPositions {
    Table,
    Code,
    IsActive,
}

#[derive(Iden)]
enum Banners {
    Table,
    PositionId,
    Status,
    Device,
    StartAt,
    EndAt,
    Priority,
    CreatedAt,
}

#[derive(Iden)]
enum BannerEvents {
    Table,
    BannerId,
    CreatedAt,
    EventType,
}
