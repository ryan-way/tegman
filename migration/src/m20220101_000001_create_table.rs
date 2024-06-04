use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Temperature::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Temperature::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Temperature::Temperature).float().not_null())
                    .col(ColumnDef::new(Temperature::Humidity).float().not_null())
                    .col(ColumnDef::new(Temperature::Hostname).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Temperature::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Temperature {
    Table,
    Id,
    Temperature,
    Humidity,
    Hostname,
}
