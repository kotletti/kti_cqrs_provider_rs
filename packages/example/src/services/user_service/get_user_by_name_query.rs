use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_provider_rs::kti_cqrs_rs::common::handler::QueryHandler;
use tokio::sync::Mutex;

use super::{user_service::User, user_service_context::UserServiceContext};

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
  type Context = UserServiceContext;
  type Output = Result<Option<User>, Box<dyn Error>>;

  async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
    let ctx = context.lock().await;
    let service = ctx.get_service();

    match service.get_user_by_name(&self.name) {
      Ok(r) => Ok(r),
      Err(e) => return Err(e.into()),
    }
  }
}
