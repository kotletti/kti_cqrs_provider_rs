use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::ports::{adapter_port::AdapterPort, context_port::ContextPort};
use kti_cqrs_provider_rs::kti_cqrs_rs::{
  errors::error::Error, ports::handler::query_handler_port::QueryHandlerPort,
};

use crate::services::user_service::{User, UserService};

pub struct GetUserByNameQuery {
  name: String,
}

impl GetUserByNameQuery {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
    }
  }
}

#[async_trait]
impl QueryHandlerPort for GetUserByNameQuery {
  type Context = Arc<dyn ContextPort>;
  type Output = Option<User>;

  async fn execute(&self, context: Self::Context) -> Result<Self::Output, Error> {
    let service = UserService::get_adapter(&context).await?;

    service.get_user_by_name(&self.name).await
  }
}
