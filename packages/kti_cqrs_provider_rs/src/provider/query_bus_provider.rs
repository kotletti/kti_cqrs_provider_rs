use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::ports::{adapter_port::AdapterPort, context_port::ContextPort};
use kti_cqrs_rs::{
  errors::error::Error,
  ports::{bus::query_bus_port::QueryBusPort, handler::query_handler_port::QueryHandlerPort},
};

pub struct QueryBusProvider {
  context: Arc<dyn ContextPort>,
}

#[async_trait]
impl AdapterPort<QueryBusProvider> for QueryBusProvider {
  fn token() -> &'static str {
    "QUERY_BUS_PROVIDER"
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

#[async_trait]
impl QueryBusPort for QueryBusProvider {
  async fn send<C: Send, O>(
    &self,
    query: Box<dyn QueryHandlerPort<Context = C, Output = O>>,
    context: C,
  ) -> Result<O, Error> {
    query.execute(context).await
  }
}

impl QueryBusProvider {
  pub fn new(context: Arc<dyn ContextPort>) -> Self {
    Self { context }
  }

  pub fn get_context(&self) -> Arc<dyn ContextPort> {
    self.context.clone()
  }
}
