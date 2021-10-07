use crate::dynamic::*;
use rbatis::rbatis::Rbatis;
use rbatis::sql;
//use rbatis::crud::CRUD;
lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[sql(RB, "select * from staff where id = ?")]
async fn select(id: i32) -> serde_json::Value {}
pub async fn connect_to_db() {
    //连接数据库,自动判断驱动类型"mysql://*","postgres://*","sqlite://*","mssql://*"加载驱动
    RB.link("mysql://root:704104@localhost:3306/test")
        .await
        .unwrap();
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();

    let data: serde_json::Value = select(1).await.unwrap();
    println!("{}", data);
    //自定义连接池参数。(可选)
    // use crate::core::db::DBPoolOptions;
    // let mut opt =DBPoolOptions::new();
    // opt.max_connections=100;
    // rb.link_opt("mysql://root:123456@localhost:3306/test",&opt).await.unwrap();
    //启用日志输出，你也可以使用其他日志框架，这个不限定的
}
