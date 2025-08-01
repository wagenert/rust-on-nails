use std::str::FromStr;

pub use cornucopia_async::Params;
pub use deadpool_postgres::{Pool, PoolError, Transaction};
pub use queries::users::User;
pub use tokio_postgres::Error as TokioPostgresError;

pub fn create_pool(db_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(db_url).unwrap();
    let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);
    deadpool_postgres::Pool::builder(manager)
        .build()
        .expect("Failed to create pool")
}

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn load_users() {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = create_pool(&db_url);

        let conn = pool.get().await.expect("Failed to get connection");

        let users = crate::queries::users::get_users()
            .bind(&conn)
            .all()
            .await
            .expect("Failed to load users");

        dbg!(users);
    }
}
