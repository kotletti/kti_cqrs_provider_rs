use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::context::{container_context::ContainerContext, context::Context};
use kti_cqrs_provider_rs::kti_cqrs_rs::common::handler::command_handler::CommandHandler;

use super::user_service::{User, UserService};

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
  type Context = Arc<ContainerContext>;
  type Output = ();

  async fn execute(&self, context: Self::Context) -> Self::Output {
    let service = context
      .resolve_provider::<UserService>(UserService::token())
      .await;

    service
      .create_user(User::new(&self.name, &self.email))
      .await
      .unwrap()
  }
}
