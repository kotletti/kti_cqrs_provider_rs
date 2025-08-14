use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::ports::{adapter_port::AdapterPort, context_port::ContextPort};
use kti_cqrs_rs::ports::{
    bus::event_bus_port::EventBusPort, handler::event_handler_port::EventHandlerPort,
};

pub struct EventBusProvider {
    context: Arc<dyn ContextPort>,
}

#[async_trait]
impl AdapterPort<EventBusProvider> for EventBusProvider {
    fn token() -> &'static str {
        "EVENT_BUS_PROVIDER"
    }
}

#[async_trait]
impl EventBusPort for EventBusProvider {
    fn send<C: Send + 'static>(&self, event: Box<dyn EventHandlerPort<Context = C>>, context: C) {
        tokio::spawn(async move {
            std::mem::drop(event.execute(context).await);
        });
    }
}

impl EventBusProvider {
    pub fn new(context: Arc<dyn ContextPort>) -> Self {
        Self { context }
    }

    pub fn get_context(&self) -> Arc<dyn ContextPort> {
        self.context.clone()
    }
}
