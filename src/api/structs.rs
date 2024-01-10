use crate::data::sqlite::structs::User;

pub struct ApiUser {
  pub id: u32,
  pub username: String,
  pub name: Option<String>,
  pub has_password: bool,
  pub is_admin: bool,
}

impl From<&User> for ApiUser {
  fn from(user: &User) -> Self {
    ApiUser {
      id: user.id,
      username: user.username.clone(),
      name: user.name.clone(),
      has_password: user.password.is_some(),
      is_admin: user.is_admin,
    }
  }
}
