use sled::transaction::ConflictableTransactionError;
use std::path::Path;

const SLED_DB_DIR_PATH: &str = "data-sled";

pub fn init_sled_db() -> sled::Db {
  let does_db_exist = Path::new(SLED_DB_DIR_PATH).exists();

  // Create or open sled database
  let db = sled::open(SLED_DB_DIR_PATH).expect("couldn't open or create sled database");
  // Create default schema for the sled database if it was just created
  if !does_db_exist {
    let init_conf_tree =
      db.open_tree("init_conf").expect("couldn't open init_conf tree in the sled database");
    init_conf_tree
      .transaction(|db| -> Result<(), ConflictableTransactionError> {
        db.insert("in_progress", bincode::serialize(&true).unwrap())?;
        db.insert("next_step", bincode::serialize(&0).unwrap())?;
        Ok(())
      })
      .expect("couldn't create default schema in the sled database");
  }

  db
}
