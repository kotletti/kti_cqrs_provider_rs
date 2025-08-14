use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::ports::{adapter_port::AdapterPort, context_port::ContextPort};
use kti_cqrs_provider_rs::kti_cqrs_rs::{
    errors::error::Error, ports::handler::event_handler_port::EventHandlerPort,
};

use crate::services::user_service::UserService;

pub struct RenameUserEvent {
    current_name: String,
    new_name: String,
}

impl RenameUserEvent {
    pub fn new(current_name: &str, new_name: &str) -> Self {
        Self {
            current_name: current_name.to_string(),
            new_name: new_name.to_string(),
        }
    }
}

#[async_trait]
impl EventHandlerPort for RenameUserEvent {
    type Context = Arc<dyn ContextPort>;

    async fn execute(&self, context: Self::Context) -> Result<(), Error> {
        let service = UserService::get_adapter(&context).await?;

        service
            .update_user_name(&self.current_name, &self.new_name)
            .await?;

        Ok(())
    }
}
