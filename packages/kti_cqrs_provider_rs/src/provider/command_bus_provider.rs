use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::{
    errors::error::Error,
    ports::{adapter_port::AdapterPort, context_port::ContextPort},
};
use kti_cqrs_rs::ports::{
    bus::command_bus_port::CommandBusPort, handler::command_handler_port::CommandHandlerPort,
};

pub struct CommandBusProvider {
    context: Arc<dyn ContextPort>,
}

#[async_trait]
impl AdapterPort<CommandBusProvider> for CommandBusProvider {
    fn token() -> &'static str {
        "COMMAND_BUS_PROVIDER"
    }
}

#[async_trait]
impl CommandBusPort for CommandBusProvider {
    async fn send<C: Send, O>(
        &self,
        command: Box<dyn CommandHandlerPort<Context = C, Output = O>>,
        context: C,
    ) -> Result<O, Error> {
        command.execute(context).await
    }
}

impl CommandBusProvider {
    pub fn new(context: Arc<dyn ContextPort>) -> Self {
        Self { context }
    }

    pub fn get_context(&self) -> Arc<dyn ContextPort> {
        self.context.clone()
    }
}
