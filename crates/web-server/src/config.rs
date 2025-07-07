#[derive(Clone, Debug)]
pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Config { db_url }
    }
}
