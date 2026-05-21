use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminAuditLogs::Table)
                    .if_not_exists()
                    .col(pk(AdminAuditLogs::Id))
                    .col(ColumnDef::new(AdminAuditLogs::ActorUserId).integer().null())
                    .col(ColumnDef::new(AdminAuditLogs::ActorName).string().not_null())
                    .col(ColumnDef::new(AdminAuditLogs::ActorEmail).string().not_null())
                    .col(ColumnDef::new(AdminAuditLogs::Action).string().not_null())
                    .col(ColumnDef::new(AdminAuditLogs::ResourceType).string().not_null())
                    .col(ColumnDef::new(AdminAuditLogs::ResourceId).integer().null())
                    .col(ColumnDef::new(AdminAuditLogs::ResourceLabel).string().null())
                    .col(ColumnDef::new(AdminAuditLogs::Message).text().null())
                    .col(timestamptz(AdminAuditLogs::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_admin_audit_logs_actor_user")
                            .from(AdminAuditLogs::Table, AdminAuditLogs::ActorUserId)
                            .to(Users::Table, Users::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_audit_logs_created_at")
                    .table(AdminAuditLogs::Table)
                    .col(AdminAuditLogs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_audit_logs_actor_user_id")
                    .table(AdminAuditLogs::Table)
                    .col(AdminAuditLogs::ActorUserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_audit_logs_action_created_at")
                    .table(AdminAuditLogs::Table)
                    .col(AdminAuditLogs::Action)
                    .col(AdminAuditLogs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_audit_logs_resource")
                    .table(AdminAuditLogs::Table)
                    .col(AdminAuditLogs::ResourceType)
                    .col(AdminAuditLogs::ResourceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(AdminAuditLogs::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

fn pk<T>(column: T) -> ColumnDef
where
    T: Iden + 'static,
{
    let mut column_def = ColumnDef::new(column);
    column_def
        .integer()
        .not_null()
        .auto_increment()
        .primary_key();
    column_def
}

fn timestamptz<T>(column: T) -> ColumnDef
where
    T: Iden + 'static,
{
    let mut column_def = ColumnDef::new(column);
    column_def.timestamp_with_time_zone().not_null();
    column_def
}

#[derive(Iden)]
enum AdminAuditLogs {
    Table,
    Id,
    ActorUserId,
    ActorName,
    ActorEmail,
    Action,
    ResourceType,
    ResourceId,
    ResourceLabel,
    Message,
    CreatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
