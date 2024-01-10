pub struct User {
  pub id: u32,
  pub username: String,
  pub name: Option<String>,
  pub password: Option<String>,
  pub is_admin: bool,
}
