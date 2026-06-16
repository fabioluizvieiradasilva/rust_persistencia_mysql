use dotenv::dotenv;
use std::env;
use mysql::{Pool, PooledConn, Opts};

pub fn get_connection() -> Result<PooledConn, mysql::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let opts = Opts::from_url(&database_url)?;
    let pool = Pool::new(opts)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}