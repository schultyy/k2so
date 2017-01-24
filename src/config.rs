#[derive(RustcEncodable)]
pub struct Config {
  pub roles: Vec<Role>
}

#[derive(RustcEncodable)]
pub struct Role {
  pub name: String,
  pub address: String
}

impl Config {
  pub fn new(role: String, address: String) -> Config {
    Config {
      roles: vec!(Role{
        name: role,
        address: address
      })
    }
  }
}