use std::sync::Arc;

use async_trait::async_trait;

use ioc_container_rs::ports::{adapter_port::AdapterPort, context_port::ContextPort};
use kti_cqrs_provider_rs::kti_cqrs_rs::{
  errors::error::Error, ports::handler::command_handler_port::CommandHandlerPort,
};

use crate::services::user_service::UserService;

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
impl CommandHandlerPort for UpdateUserCommand {
  type Context = Arc<dyn ContextPort>;
  type Output = ();

  async fn execute(&self, context: Self::Context) -> Result<Self::Output, Error> {
    let service = UserService::get_adapter(&context).await?;

    service.update_user_email(&self.name, &self.email).await?;

    Ok(())
  }
}
