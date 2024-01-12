use crate::AppState;
use actix_web::web;
use serde::Deserialize;
use sled::transaction::ConflictableTransactionError;
use std::path::Path;

const SLED_DB_DIR_PATH: &str = "data-sled";

pub fn get_from_state<T: for<'a> Deserialize<'a>>(
  app_state: &web::Data<AppState>,
  tree_name: &str,
  key: &str,
) -> T {
  let tree = app_state
    .sled_db
    .open_tree(tree_name)
    .expect(&format!("couldn't open {} tree in the sled database", tree_name));
  bincode::deserialize::<T>(
    &tree
      .get(key)
      .expect(&format!("couldn't get {} key from {} tree in the sled database", key, tree_name))
      .expect(&format!("{} value is None", key)),
  )
  .expect(&format!("couldn't deserialize {} value", key))
}

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
        db.insert("is_in_progress", bincode::serialize(&true).unwrap())?;
        db.insert("next_step", bincode::serialize(&0).unwrap())?;
        Ok(())
      })
      .expect("couldn't create default schema in the sled database");
  }

  db
}
