// use dotenv::dotenv;
// use mysql::*;

// pub struct SQLDatabase {
//     pub conn: PooledConn,
// }

// impl SQLDatabase {
//     pub fn new() -> Self {
//         dotenv().ok();
//         let url = std::env::var("DATABASE_URL").expect("DB URL Env Not Found");
//         let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
//         let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
//         let conn = pool.get_conn().expect("Failed To Get DB Connection");

//         Self { conn }
//     }
// }
