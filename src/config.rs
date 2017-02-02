#[derive(Default)]
#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
pub struct Config {
  pub roles: Vec<Role>
}

#[derive(Default)]
#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
pub struct Role {
  pub name: String,
  pub address: String
}

impl Config {
  pub fn new() -> Config {
    Default::default()
  }

  pub fn add_role(&mut self, role: String, address: String) {
    self.roles.push(Role{
      name: role,
      address: address
    });
  }
}