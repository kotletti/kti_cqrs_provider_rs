use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::{
  errors::error::Error,
  ports::{adapter_port::AdapterPort, context_port::ContextPort},
};
use kti_cqrs_rs::ports::{
  bus::{
    command_bus_port::CommandBusPort, event_bus_port::EventBusPort, query_bus_port::QueryBusPort,
    service_bus_port::ServiceBusPort,
  },
  handler::{
    command_handler_port::CommandHandlerPort, event_handler_port::EventHandlerPort,
    query_handler_port::QueryHandlerPort,
  },
};

use super::{
  command_bus_provider::CommandBusProvider, event_bus_provider::EventBusProvider,
  query_bus_provider::QueryBusProvider,
};

pub struct CqrsProvider {
  context: Arc<dyn ContextPort>,
}

#[async_trait]
impl AdapterPort<CqrsProvider> for CqrsProvider {
  fn token() -> &'static str {
    "CQRS_PROVIDER"
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

impl CqrsProvider {
  pub fn new(context: Arc<dyn ContextPort>) -> Self {
    Self { context }
  }

  pub fn get_context(&self) -> Arc<dyn ContextPort> {
    self.context.clone()
  }
}

#[async_trait]
impl ServiceBusPort for CqrsProvider {
  type Context = Arc<dyn ContextPort>;

  async fn event(
    &self,
    event: Box<dyn EventHandlerPort<Context = Self::Context>>,
  ) -> Result<(), Error> {
    let bus = EventBusProvider::get_adapter(&self.get_context()).await?;

    bus.send(event, self.get_context());

    Ok(())
  }

  async fn command<O>(
    &self,
    command: Box<dyn CommandHandlerPort<Context = Self::Context, Output = O>>,
  ) -> Result<O, Error> {
    let bus = CommandBusProvider::get_adapter(&self.get_context()).await?;

    bus.send(command, self.get_context()).await
  }

  async fn query<O>(
    &self,
    query: Box<dyn QueryHandlerPort<Context = Self::Context, Output = O>>,
  ) -> Result<O, Error> {
    let bus = QueryBusProvider::get_adapter(&self.get_context()).await?;

    bus.send(query, self.get_context()).await
  }
}
