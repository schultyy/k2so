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

  pub fn is_role_unique(&self, role_name: &str) -> bool {
    !self.roles.iter().any(|ref role| role.name == role_name)
  }

  pub fn address_for_role_name(&self, role_name: &str) -> Option<String> {
    match self.roles.iter().find(|&r| r.name == role_name) {
      Some(role) => Some(role.address.clone()),
      None => None
    }
  }
}