const SLED_DB_DIR_PATH: &str = "data-sled";

pub fn init_sled_db() -> sled::Db {
  // Create or open sled database and return it
  sled::open(SLED_DB_DIR_PATH).expect("couldn't open or create sled database")
}
