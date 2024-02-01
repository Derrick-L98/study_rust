
use mysql::prelude::{WithParams, BinQuery};
use chrono::prelude::*;
use mysql_async::prelude::*;
use rust_decimal::Decimal;
use rbatis::{
    crud::{Skip, CRUD},
};
use rbatis::rbatis::Rbatis;

use crate::sql::{ Sql, RbSql, SeaSql};

use anyhow::{anyhow, Result};
#[crud_table(id_name:"id"|id_type:"i64"|table_name:"sys_commodity_group_list")]
#[derive(Clone, Debug, Default)]
pub struct  SysCommodityGroupList {
   pub id: i64,                 // bigint(20) NOT NULL AUTO_INCREMENT,
   pub group_id: i64,           // bigint(20) NOT NULL DEFAULT '0' COMMENT '分组ID',
   pub commodity_id: i64,       // bigint(20) NOT NULL DEFAULT '0' COMMENT '品种ID',
}



// #[crud_table(id_name:"vc_system_codepub"|id_type:"String"|table_name:"tsystempub")]
#[derive(Clone, Debug, Default)]
pub struct System {
    pub vc_system_codepub: String,//  varchar(6) NOT NULL,
    pub vc_system_namepub: String,//  varchar(64) NOT NULL,
    pub l_preinit_datepub: Decimal,// decimal(8,0) NOT NULL,
    pub l_init_datepub: Decimal,  //decimal(8,0) NOT NULL,
    pub en_time_zonepub: Decimal,  //decimal(3,1) NOT NULL,
    pub vc_current_versionpub: String,//  varchar(16) DEFAULT NULL,
    pub vc_tran_current_versionpub: String,//  varchar(16) DEFAULT NULL,
}
impl System {
    // pub async fn rb_query(id: i64, db: &RbSql) -> Option<System> {
    //     // let w = db
    //     //     .get_connection()
    //     //     .new_wrapper()
    //     //     .eq("id", id); //

    //     // let query_ret = db.get_connection().fetch_by_wrapper(w).await;
    //     // // select * from phoenix_account_assets where id = ?;

    //     // match query_ret {
    //     //     Ok(v) => v,
    //     //     Err(e) => {
    //     //         println!("query error:{}", e);
    //     //         None
    //     //     }
    //     // }
    // }

    pub async fn sql_query(db: &Sql) {//-> Option<System> {
        let mut conn = db.get_connection().get_conn().await.unwrap();
        

        let sql = "SELECT id FROM sys_commodity_group WHERE inner_code = '1002_HK'";
        
        let t2 = Local::now().naive_local().time();
        // let mut s  = sql.run(&mut conn).await.unwrap();
        let result: Vec<i64> = conn.exec(sql, ()).await.unwrap();
        if result.is_empty() {
            print!("NULL");
        }
        let t3 = Local::now().naive_local().time();
        println!("Sql查询时间  : {}",t3 - t2);
        // let result_set: Vec<String> = s.collect().await.unwrap();
        // for val in result_set {
        //     println!("inner_code = {}\n",val);
        // }
        
        // let f = s.();
        // print!("{:#?}",f);

        // let ret: Vec<String> = s.collect().await.unwrap();
        // for val in ret {
        //     println!("id = {}\n",val);
        //     break;
        // }

        // let sql = "SELECT commodity_id FROM sys_commodity_group_list 
	    //             WHERE group_id = ( SELECT id FROM sys_commodity_group WHERE inner_code = '1002_HK' )";
        // let t2 = Local::now().naive_local().time();
        // let result: Vec<i64> = conn.exec(sql, ()).await.unwrap();
        // let t3 = Local::now().naive_local().time();
        // println!("Sql查询时间  : {}",t3 - t2);

        
        // let result: Vec<String> = conn.exec("SELECT inner_code FROM sys_commodity_group WHERE type = 3 and status = 1", ()).await.unwrap();
        // for val in result {
        //     println!("inner_code = {}\n",val);
        // }
    }

}


impl SysCommodityGroupList {
    pub fn new() -> Self {
        SysCommodityGroupList { ..Default::default() }
    }

    pub async fn query_many(group_id: i64, dbconn: &RbSql) -> Result<Vec<SysCommodityGroupList>> {
        let w = dbconn
            .get_connection()
            .new_wrapper()
            .do_if(group_id > 0, |w| w.eq("group_id", group_id)); //

        let t_cr = Local::now().naive_local().time();
        let query_ret = dbconn.get_connection().fetch_list_by_wrapper(w).await;
        let t1_cr = Local::now().naive_local().time();
        println!("RbSql查询时间: {}",t1_cr - t_cr);

        // log::info!("查找分帐户持仓信息结果:{:?}", query_ret);

        let ret = query_ret.map_or_else(
            |e| {
                Err(anyhow!("查找品种分组关联商品的明细：{}", e.to_string()))
            },
            |v| Ok(v),
        );
        ret
    }
    
}