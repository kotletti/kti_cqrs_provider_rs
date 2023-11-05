use std::{
  error::Error,
  sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct User {
  name: String,
  email: String,
}

impl User {
  pub fn new(name: &str, email: &str) -> Self {
    Self {
      name: name.to_string(),
      email: email.to_string(),
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_email(&self) -> &str {
    &self.email
  }
}

#[derive(Clone)]
pub struct UserService {
  users: Arc<Mutex<Vec<User>>>,
}

impl UserService {
  pub fn new(users: Vec<User>) -> Self {
    Self {
      users: Arc::new(Mutex::new(users)),
    }
  }

  pub fn get_user_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn Error>> {
    let users = self.users.lock().unwrap();

    let user = users.iter().find(|i| i.name.eq(name)).cloned();

    Ok(user)
  }

  pub fn create_user(&self, user: User) -> Result<(), Box<dyn Error>> {
    let mut users = self.users.lock().unwrap();

    users.push(user);

    Ok(())
  }

  pub fn update_user_email(&self, name: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let mut users = self.users.lock().unwrap();

    let index = match users.iter().position(|i| i.name == name) {
      Some(r) => r,
      None => return Err("Cant find user by name.".into()),
    };

    users.remove(index);

    users.push(User::new(name, email));

    Ok(())
  }
}
