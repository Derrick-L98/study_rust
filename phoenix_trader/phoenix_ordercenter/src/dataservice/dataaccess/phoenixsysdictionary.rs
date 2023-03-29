use anyhow::{anyhow, Result};
use sea_orm::entity::prelude::*;
use sea_orm::{entity::*, Condition, QueryFilter, QueryOrder, QuerySelect};
use crate::dataservice::entities::{prelude::*, *};
use crate::client::dbclient::DbClient;

//数字字典表
impl PhoenixSysDictionary {
    // pub async fn query_dictionary(db: &DbClient) -> Result<Vec<PhoenixSysDictionary>>{
    //     let db = db.get_conn().await;
    //     match PhoenixSysDictionaryEntity::find()
    //         .filter(phoenix_sys_dictionary::Column::DictionaryNo.eq(10028))
    //         .filter(phoenix_sys_dictionary::Column::LemmaItem.is_not_in(vec!["!", "0", "8"]))
    //         .all(db)
    //     .await {
    //         Ok(v) => {
    //             return Ok(v);
    //         },
    //         Err(err) => {
    //             log::error!("{:?}", err);
    //             return Err(anyhow!("{:?}", err));
    //         }
    //     }
    // }

    pub async fn query_dictionary(db: &DbClient) -> Result<Vec<PhoenixSysDictionary>>{
        let db = db.get_conn();
        // Condition::any() == > or
        // Condition::all() ==> and
        match PhoenixSysDictionaryEntity::find()
            .filter(
                Condition::all()
                .add(phoenix_sys_dictionary::Column::DictionaryNo.eq(10028))
                .add(phoenix_sys_dictionary::Column::LemmaItem.is_not_in(vec!["!", "0", "8"]))
            )
            .all(db)
        .await {
            Ok(v) => {
                return Ok(v);
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        }
    }
}
