use r2d2_postgres::{SslMode, PostgresConnectionManager};
use r2d2::{Pool, Config};
use iron::typemap::Key;


pub struct Db;

impl Db {
    pub fn get_connection_pool(url: &str) -> Pool<PostgresConnectionManager>{
        debug!("Getting connection pool");
        let r2d2_config = Config::default();
        let r2d2_manager = PostgresConnectionManager::new(url, SslMode::None)
                           .expect("Could not setup Postgres Connection Manager");

        Pool::new(r2d2_config, r2d2_manager).expect("Could not create connection pool")
    }
}

impl Key for Db {
    type Value = Pool<PostgresConnectionManager>;
}
