use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::{Pool, PooledConnection};
use iron::typemap::Key;
use iron::prelude::*;
use persistent::{Write};


pub struct Db;

impl Db {
    pub fn get_connection_pool(url: &str) -> Pool<PostgresConnectionManager>{
        debug!("Getting connection pool");
        let r2d2_manager = PostgresConnectionManager::new(url, TlsMode::None)
                           .expect("Could not setup Postgres Connection Manager");

        Pool::new(r2d2_manager).expect("Could not create connection pool")
    }

    pub fn from_request(req: &mut Request) -> PooledConnection<PostgresConnectionManager> {
        let mutex = req.get::<Write<Db>>().expect("Could not get mutex on connection pool");
        let pool = mutex.lock().expect("Could not lock mutex on connection pool");

        pool.get().expect("Couldn't get database from connection pool")
    }
}

impl Key for Db {
    type Value = Pool<PostgresConnectionManager>;
}
