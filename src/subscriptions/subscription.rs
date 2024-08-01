use std::sync::Arc;

use serde::{Deserialize, Serialize};
use service_sdk::my_postgres::macros::{
    DbEnumAsString, InsertDbEntity, SelectDbEntity, TableSchema, UpdateDbEntity, WhereDbModel,
};
use service_sdk::my_postgres::{self, MyPostgres, MyPostgresError, PostgresSettings};

#[derive(Debug, Clone, Copy, DbEnumAsString, Serialize, Deserialize)]
pub enum CopyTradingSubscriptionDbStatus {
    Active,
    Paused,
}

#[derive(InsertDbEntity, SelectDbEntity, Debug, Clone, TableSchema, Serialize, Deserialize)]
pub struct CopyTradingSubscriptionDbModel {
    #[primary_key]
    pub id: String,
    #[db_index(id:0, index_name:"copy_trading_subscription_provider_id_index", is_unique:false, order:"ASC")]
    pub provider_id: String,
    pub trader_id: String,
    pub account_id: String,
    pub status: CopyTradingSubscriptionDbStatus,
    pub copy_trading_coefficient: f64,
    pub pl_force_stop_loss: Option<f64>,
}

#[derive(UpdateDbEntity, Debug, Clone, Serialize, Deserialize)]
pub struct CopyTradingSubscriptionUpdateDbModel {
    pub id: String,
    pub status: CopyTradingSubscriptionDbStatus,
    pub copy_trading_coefficient: f64,
    pub pl_force_stop_loss: Option<f64>,
}

#[derive(WhereDbModel, Debug, Clone, Serialize, Deserialize)]
pub struct CopyTradingSubscriptionWhereDbModel {
    #[ignore_if_none]
    pub id: Option<String>,
    #[ignore_if_none]
    pub provider_id: Option<String>,
}

pub const TABLE_NAME: &str = "copy_trading_subscription";
const PK: &str = "copy_trading_subscription_pk";
pub struct CopyTradingSubscriptionDbRepository {
    postgres: my_postgres::MyPostgres,
}

impl CopyTradingSubscriptionDbRepository {
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
            .with_table_schema_verification::<CopyTradingSubscriptionDbModel>(
                TABLE_NAME,
                Some(PK.to_string()),
            )
            .build()
            .await,
        }
    }

    pub async fn add_subscription(
        &self,
        model: CopyTradingSubscriptionDbModel,
    ) -> Result<(), MyPostgresError> {
        self.postgres
            .insert_db_entity(&model, TABLE_NAME, None)
            .await?;

        Ok(())
    }

    pub async fn update_subscription(
        &self,
        update_model: CopyTradingSubscriptionUpdateDbModel,
    ) -> Result<(), MyPostgresError> {
        self.postgres
            .update_db_entity(&update_model, TABLE_NAME, None)
            .await?;

        Ok(())
    }

    pub async fn query_subscription(
        &self,
        query: Option<&CopyTradingSubscriptionWhereDbModel>,
    ) -> Result<Vec<CopyTradingSubscriptionDbModel>, MyPostgresError> {
        self.postgres.query_rows(TABLE_NAME, query, None).await
    }
}
