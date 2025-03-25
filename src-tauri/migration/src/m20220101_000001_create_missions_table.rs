// m20220101_000001_create_missions_table.rs
use sea_orm_migration::prelude::*;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::extension::postgres::Type;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220101_000001_create_missions_table" // Make sure this matches with the file name
  }
}

#[derive(Iden, EnumIter)]
pub enum Statuses {
  #[iden = "Active"]
  Active,
  #[iden = "Inactive"]
  Inactive,
  #[iden = "Complete"]
  Complete,
  #[iden = "Failed"]
  Failed,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  // Define how to apply this migration: Create the Bakery table.
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
    .create_type(
        Type::create()
            .as_enum(Alias::new("status"))
            .values([Alias::new("Active"), Alias::new("Inactive"), Alias::new("Complete"), Alias::new("Failed")])
            .to_owned(),
    )
    .await?;
    manager.create_table(
      Table::create()
        .table(Missions::Table)
        .col(ColumnDef::new(Missions::Name).string().not_null().primary_key())
        .col(ColumnDef::new(Missions::KeepInZones).array(ColumnType::Text).not_null())
        .col(ColumnDef::new(Missions::KeepOutZones).array(ColumnType::Text).not_null())
        .col(ColumnDef::new(Missions::Status).enumeration(Alias::new("status"), Statuses::iter()))
        .to_owned()
    ).await
  }

  // Define how to rollback this migration: Drop the Missions table.
  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table(Table::drop().table(Missions::Table).to_owned()).await
  }
}

#[derive(Iden)]
pub enum Missions {
  Table,
  Name,
  KeepInZones,
  KeepOutZones,
  Status,
}
