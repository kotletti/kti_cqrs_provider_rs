use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{
  common::{
    context::Context,
    handler::{CommandHandler, QueryHandler},
  },
  core::bus::{CommandBus, QueryBus, ServiceBus},
};
use tokio::sync::Mutex;

pub const TOKEN_PROVIDER: &'static str = "CQRS_SERVICE_CONTAINER_PROVIDER";

#[derive(Clone)]
pub struct Provider<C: Context> {
  provider: &'static str,
  context: Arc<Mutex<C>>,
  command_bus: CommandBus,
  query_bus: QueryBus,
}

impl<C: Context> Provider<C> {
  pub fn new(context: Arc<Mutex<C>>) -> Self {
    Provider {
      provider: TOKEN_PROVIDER,
      context,
      command_bus: CommandBus,
      query_bus: QueryBus,
    }
  }

  pub fn get_token_provider(&self) -> &'static str {
    self.provider
  }

  pub fn get_context(&self) -> Arc<Mutex<C>> {
    self.context.clone()
  }
}

#[async_trait]
impl<C: Context> ServiceBus for Provider<C> {
  type Context = C;

  async fn command<O>(&self, command: Box<dyn CommandHandler<Context = C, Output = O>>) -> O {
    self.command_bus.send(command, self.get_context()).await
  }

  async fn query<O>(&self, query: Box<dyn QueryHandler<Context = Self::Context, Output = O>>) -> O {
    self.query_bus.send(query, self.get_context()).await
  }
}
