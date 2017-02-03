#[derive(Default)]
#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
pub struct Config {
  pub roles: Vec<Role>,
  pub username: String,
  pub ssh_key_path: String
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

  pub fn add_username(&mut self, username: String) {
    self.username = username;
  }

  pub fn add_ssh_key(&mut self, ssh_key_path: String) {
    self.ssh_key_path = ssh_key_path;
  }

  pub fn is_valid(&self) -> Result<(), Vec<String>> {
    let mut is_valid = true;
    let mut error_messages = vec!();

    if self.username.len() == 0 {
      is_valid = false;
      error_messages.push("Username must be set".into());
    }
    if self.ssh_key_path.len() == 0 {
      is_valid = false;
      error_messages.push("SSH Key path must be set".into());
    }

    if is_valid {
      return Ok(())
    } else {
      return Err(error_messages)
    }
  }
}