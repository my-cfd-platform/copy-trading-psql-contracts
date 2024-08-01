use std::sync::Arc;

use serde::{Deserialize, Serialize};
use service_sdk::my_postgres::macros::{DbEnumAsString, InsertDbEntity, SelectDbEntity, TableSchema, WhereDbModel};
use service_sdk::my_postgres::{self, MyPostgres, MyPostgresError, PostgresSettings};

#[derive(Debug, Clone, DbEnumAsString, Copy, Serialize, Deserialize)]
pub enum CopyTradingPositionType{
    Market,
    Pending

}
#[derive(InsertDbEntity, SelectDbEntity, Debug, Clone, TableSchema, Serialize, Deserialize)]
pub struct CopyTradingPositionDbModel {
    #[primary_key]
    pub id: String,
    pub provider_id: String,
    pub subscription_id: String,
    pub source_position_id: String,
    pub position_type: CopyTradingPositionType
}

#[derive(WhereDbModel, Debug, Clone, TableSchema, Serialize, Deserialize)]
pub struct CopyTradingPositionWhereDbModel {
    #[ignore_if_none]
    pub id: Option<Vec<String>>,
    #[ignore_if_none]
    pub source_position_id: Option<String>,
    #[ignore_if_none]
    pub subscription_id: Option<String>,
}

pub const TABLE_NAME: &str = "copy_trading_position";
const PK: &str = "copy_trading_position_pk";

pub struct CopyTradingPositionsDbRepository {
    postgres: my_postgres::MyPostgres,
}

impl CopyTradingPositionsDbRepository {
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
            .with_table_schema_verification::<CopyTradingPositionDbModel>(
                TABLE_NAME,
                Some(PK.to_string()),
            )
            .build()
            .await,
        }
    }

    pub async fn add_positions(
        &self,
        positions: Vec<CopyTradingPositionDbModel>,
    ) -> Result<(), MyPostgresError> {
        self.postgres
            .bulk_insert_db_entities(&positions, TABLE_NAME, None)
            .await?;

        Ok(())
    }

    pub async fn delete_positions(
        &self,
        query: CopyTradingPositionWhereDbModel,
    ) -> Result<(), MyPostgresError> {
        self.postgres
            .bulk_delete(TABLE_NAME, &[query], None)
            .await?;

        Ok(())
    }

    pub async fn query_positions(
        &self,
        query: Option<CopyTradingPositionWhereDbModel>,
    ) -> Result<Vec<CopyTradingPositionDbModel>, MyPostgresError> {
        self.postgres
            .query_rows(TABLE_NAME, query.as_ref(), None)
            .await
    }
}
