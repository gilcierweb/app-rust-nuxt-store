use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
     
        #[derive(Iden)]
        enum Categories {
            Table,
            Id,
            ParentId,
        }

        manager
            .alter_table(
                Table::alter()
                    .table(Categories::Table)
                    .add_column(ColumnDef::new(Categories::ParentId).integer().null())
                    .to_owned(),
            )
            .await?;
      
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_parent_category")
                    .from(Categories::Table, Categories::ParentId)
                    .to(Categories::Table, Categories::Id) // Corrigido para Categories::Id
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
     
        #[derive(Iden)]
        enum Categories {
            Table,
            ParentId,
        }
       
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_parent_category")
                    .table(Categories::Table)
                    .to_owned(),
            )
            .await?;
    
        manager
            .alter_table(
                Table::alter()
                    .table(Categories::Table)
                    .drop_column(Categories::ParentId)
                    .to_owned(),
            )
            .await
    }
}
