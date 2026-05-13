use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Roles::Name).string().not_null())
                    .col(ColumnDef::new(Roles::ResourceType).string().null())
                    .col(ColumnDef::new(Roles::ResourceId).integer().null())
                    .col(
                        ColumnDef::new(Roles::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Roles::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UsersRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UsersRoles::UserId).integer().not_null())
                    .col(ColumnDef::new(UsersRoles::RoleId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(UsersRoles::UserId)
                            .col(UsersRoles::RoleId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_roles_user")
                            .from(UsersRoles::Table, UsersRoles::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_roles_role")
                            .from(UsersRoles::Table, UsersRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_roles_on_name_and_resource")
                    .table(Roles::Table)
                    .col(Roles::Name)
                    .col(Roles::ResourceType)
                    .col(Roles::ResourceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_roles_on_user_id_and_role_id")
                    .table(UsersRoles::Table)
                    .col(UsersRoles::UserId)
                    .col(UsersRoles::RoleId)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        db.execute_unprepared(
            "INSERT INTO roles (name, resource_type, resource_id, created_at, updated_at) VALUES ('admin', NULL, NULL, NOW(), NOW())",
        )
        .await?;
        db.execute_unprepared(
            "INSERT INTO roles (name, resource_type, resource_id, created_at, updated_at) VALUES ('user', NULL, NULL, NOW(), NOW())",
        )
        .await?;
        db.execute_unprepared(
            "INSERT INTO users_roles (user_id, role_id) SELECT users.id, roles.id FROM users JOIN roles ON roles.name = 'admin' AND roles.resource_type IS NULL AND roles.resource_id IS NULL WHERE users.id = 1 ON CONFLICT DO NOTHING",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UsersRoles::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Roles {
    Table,
    Id,
    Name,
    ResourceType,
    ResourceId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum UsersRoles {
    Table,
    UserId,
    RoleId,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
