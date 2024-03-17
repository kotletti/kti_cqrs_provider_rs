use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::context::{container_context::ContainerContext, context::Context};
use kti_cqrs_rs::{
  common::handler::{command_handler::CommandHandler, query_handler::QueryHandler},
  core::bus::service_bus::ServiceBus,
};

use super::{command_bus_provider::CommandBusProvider, query_bus_provider::QueryBusProvider};

pub struct Provider {
  context: Arc<ContainerContext>,
}

impl Provider {
  pub fn new(context: Arc<ContainerContext>) -> Self {
    Self { context }
  }

  pub fn token() -> &'static str {
    "CQRS_PROVIDER"
  }

  pub fn get_context(&self) -> Arc<ContainerContext> {
    self.context.clone()
  }
}

#[async_trait]
impl ServiceBus for Provider {
  type Context = Arc<ContainerContext>;

  async fn command<O>(
    &self,
    command: Box<dyn CommandHandler<Context = Self::Context, Output = O>>,
  ) -> O {
    let bus = self
      .context
      .resolve_provider::<CommandBusProvider>(CommandBusProvider::token())
      .await;

    bus.send(command).await
  }

  async fn query<O>(&self, query: Box<dyn QueryHandler<Context = Self::Context, Output = O>>) -> O {
    let bus = self
      .context
      .resolve_provider::<QueryBusProvider>(QueryBusProvider::token())
      .await;

    bus.send(query).await
  }
}
