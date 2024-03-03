use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_provider_rs::kti_cqrs_rs::common::handler::CommandHandler;
use tokio::sync::Mutex;

use super::user_service_context::UserServiceContext;

pub struct UpdateUserCommand {
  name: String,
  email: String,
}

impl UpdateUserCommand {
  pub fn new(name: &str, email: &str) -> Self {
    Self {
      name: name.to_string(),
      email: email.to_string(),
    }
  }
}

#[async_trait]
impl CommandHandler for UpdateUserCommand {
  type Context = UserServiceContext;
  type Output = ();

  async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
    let ctx = context.lock().await;
    let service = ctx.get_service();

    service.update_user_email(&self.name, &self.email).unwrap()
  }
}
