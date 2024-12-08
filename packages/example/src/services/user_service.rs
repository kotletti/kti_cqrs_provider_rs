use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::ports::{adapter_port::AdapterPort, context_port::ContextPort};
use kti_cqrs_provider_rs::kti_cqrs_rs::errors::error::Error;
use tokio::sync::RwLock;

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
  users: Arc<RwLock<Vec<User>>>,
}

#[async_trait]
impl AdapterPort<UserService> for UserService {
  fn token() -> &'static str {
    "USER_SERVICE"
  }

  async fn get_adapter(context: &Arc<dyn ContextPort>) -> Result<Box<Self>, Error> {
    let me = context
      .resolve_provider(Self::token())
      .await?
      .downcast::<Self>()
      .map_err(|_| format!("Cant resolve provider: {}", Self::token()))?;

    Ok(me)
  }
}

impl UserService {
  pub fn new(users: Arc<RwLock<Vec<User>>>) -> Self {
    Self { users }
  }

  pub async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, Error> {
    let users = self.users.read().await;

    let user = users.iter().find(|i| i.name.eq(name)).cloned();

    Ok(user)
  }

  pub async fn create_user(&self, user: User) -> Result<(), Error> {
    let mut users = self.users.write().await;

    users.push(user);

    Ok(())
  }

  pub async fn update_user_email(&self, name: &str, email: &str) -> Result<(), Error> {
    let mut users = self.users.write().await;

    let index = match users.iter().position(|i| i.name == name) {
      Some(r) => r,
      None => return Err("Cant find user by name.".into()),
    };

    users.remove(index);

    users.push(User::new(name, email));

    Ok(())
  }

  pub async fn update_user_name(&self, current_name: &str, new_name: &str) -> Result<(), Error> {
    let mut users = self.users.write().await;

    let index = match users.iter().position(|i| i.name == current_name) {
      Some(r) => r,
      None => return Err("Cant find user by name.".into()),
    };

    let user = users[index].clone();

    users.remove(index);

    users.push(User::new(new_name, user.get_email()));

    Ok(())
  }
}
