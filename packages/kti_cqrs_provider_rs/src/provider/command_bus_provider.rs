use std::sync::Arc;

use ioc_container_rs::context::container_context::ContainerContext;
use kti_cqrs_rs::{
  common::handler::command_handler::CommandHandler, core::bus::command_bus::CommandBus,
};

pub struct CommandBusProvider {
  context: Arc<ContainerContext>,
  bus: Arc<CommandBus>,
}

impl CommandBusProvider {
  pub fn new(context: Arc<ContainerContext>, bus: Arc<CommandBus>) -> Self {
    Self { context, bus }
  }

  pub fn token() -> &'static str {
    "COMMAND_BUS_PROVIDER"
  }

  pub fn get_context(&self) -> Arc<ContainerContext> {
    self.context.clone()
  }

  pub async fn send<O>(
    &self,
    command: Box<dyn CommandHandler<Context = Arc<ContainerContext>, Output = O>>,
  ) -> O {
    self.bus.send(command, self.get_context()).await
  }
}
