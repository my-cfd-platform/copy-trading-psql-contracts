use std::sync::Arc;

use serde::{Deserialize, Serialize};
use service_sdk::my_postgres::macros::{
    DbEnumAsString, InsertDbEntity, SelectDbEntity, TableSchema, UpdateDbEntity, WhereDbModel,
};
use service_sdk::my_postgres::{self, MyPostgres, MyPostgresError, PostgresSettings};
use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Clone, Copy, DbEnumAsString, Serialize, Deserialize)]
pub enum CopyTradingProviderDbStatus {
    Active,
    Paused,
    Disabled,
}

#[derive(InsertDbEntity, SelectDbEntity, Debug, Clone, TableSchema, Serialize, Deserialize)]
pub struct CopyTradingProviderDbModel {
    #[primary_key]
    pub id: String,
    pub trader_id: String,
    pub account_id: String,
    pub status: CopyTradingProviderDbStatus,
    #[sql_type("timestamp")]
    pub create_date: DateTimeAsMicroseconds,
}

#[derive(UpdateDbEntity, Debug, Clone, Serialize, Deserialize)]
pub struct CopyTradingProviderUpdateDbModel {
    pub id: String,
    pub status: CopyTradingProviderDbStatus,
}
#[derive(WhereDbModel, Debug, Clone, Serialize, Deserialize)]
pub struct CopyTradingProviderWhereDbModel {
    #[ignore_if_none]
    pub id: Option<String>,
    #[ignore_if_none]
    pub trader_id: Option<String>,
    #[ignore_if_none]
    pub account_id: Option<String>,
}

pub const TABLE_NAME: &str = "copy_trading_provider";
const PK: &str = "copy_trading_provider_pk";
pub struct CopyTradingProviderDbRepository {
    postgres: my_postgres::MyPostgres,
}

impl CopyTradingProviderDbRepository {
    pub async fn new(
        postgres: Arc<dyn PostgresSettings + Sync + Send + 'static>,
        service_name: String,
    ) -> Self {
        Self {
            postgres: MyPostgres::from_settings(
                service_name,
                postgres.clone(),
                service_sdk::my_logger::LOGGER.clone(),
            )
            .with_table_schema_verification::<CopyTradingProviderDbModel>(
                TABLE_NAME,
                Some(PK.to_string()),
            )
            .build()
            .await,
        }
    }

    pub async fn add_provider(
        &self,
        model: CopyTradingProviderDbModel,
    ) -> Result<(), MyPostgresError> {
        self.postgres
            .insert_db_entity(&model, TABLE_NAME, None)
            .await?;

        Ok(())
    }

    pub async fn update_provider(
        &self,
        update_model: CopyTradingProviderUpdateDbModel,
    ) -> Result<(), MyPostgresError> {
        self.postgres
            .update_db_entity(&update_model, TABLE_NAME, None)
            .await?;

        Ok(())
    }

    pub async fn query_provider(
        &self,
        query: Option<CopyTradingProviderWhereDbModel>,
    ) -> Result<Vec<CopyTradingProviderDbModel>, MyPostgresError> {
        self.postgres
            .query_rows(TABLE_NAME, query.as_ref(), None)
            .await
    }
}
