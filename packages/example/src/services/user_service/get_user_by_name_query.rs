use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use ioc_container_rs::context::{container_context::ContainerContext, context::Context};
use kti_cqrs_provider_rs::kti_cqrs_rs::common::handler::query_handler::QueryHandler;

use super::user_service::{User, UserService};

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
impl QueryHandler for GetUserByNameQuery {
  type Context = Arc<ContainerContext>;
  type Output = Result<Option<User>, Box<dyn Error>>;

  async fn execute(&self, context: Self::Context) -> Self::Output {
    let service = context
      .resolve_provider::<UserService>(UserService::token())
      .await;

    match service.get_user_by_name(&self.name).await {
      Ok(r) => Ok(r),
      Err(e) => return Err(e.into()),
    }
  }
}
