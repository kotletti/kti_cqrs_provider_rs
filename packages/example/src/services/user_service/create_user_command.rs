use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use kti_cqrs_rs::common::handler::CommandHandler;

use super::{user_service::User, user_service_context::UserServiceContext};

pub struct CreateUserCommand {
  name: String,
  email: String,
}

impl CreateUserCommand {
  pub fn new(name: &str, email: &str) -> Self {
    Self {
      name: name.to_string(),
      email: email.to_string(),
    }
  }
}

#[async_trait]
impl CommandHandler for CreateUserCommand {
  type Context = UserServiceContext;
  type Output = ();

  async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
    let ctx = context.lock().unwrap();

    let service = ctx.get_service().lock().unwrap().clone();

    service
      .create_user(User::new(&self.name, &self.email))
      .unwrap()
  }
}
