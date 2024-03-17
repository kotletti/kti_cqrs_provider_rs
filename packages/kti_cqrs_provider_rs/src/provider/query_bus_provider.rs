use std::sync::Arc;

use ioc_container_rs::context::container_context::ContainerContext;
use kti_cqrs_rs::{common::handler::query_handler::QueryHandler, core::bus::query_bus::QueryBus};

pub struct QueryBusProvider {
  context: Arc<ContainerContext>,
  bus: Arc<QueryBus>,
}

impl QueryBusProvider {
  pub fn new(context: Arc<ContainerContext>, bus: Arc<QueryBus>) -> Self {
    Self { context, bus }
  }

  pub fn token() -> &'static str {
    "QUERY_BUS_PROVIDER"
  }

  pub fn get_context(&self) -> Arc<ContainerContext> {
    self.context.clone()
  }

  pub async fn send<O>(
    &self,
    query: Box<dyn QueryHandler<Context = Arc<ContainerContext>, Output = O>>,
  ) -> O {
    self.bus.send(query, self.get_context()).await
  }
}
