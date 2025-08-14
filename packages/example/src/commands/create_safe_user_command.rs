use std::sync::Arc;

use crate::{
    queries::get_user_by_name_query::GetUserByNameQuery,
    services::user_service::{User, UserService},
};
use async_trait::async_trait;
use ioc_container_rs::ports::{adapter_port::AdapterPort, context_port::ContextPort};
use kti_cqrs_provider_rs::kti_cqrs_rs::ports::bus::service_bus_port::ServiceBusPort;
use kti_cqrs_provider_rs::{
    kti_cqrs_rs::{errors::error::Error, ports::handler::command_handler_port::CommandHandlerPort},
    provider::cqrs_provider::CqrsProvider,
};

pub struct CreateSafeUserCommand {
    name: String,
    email: String,
}

impl CreateSafeUserCommand {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}

#[async_trait]
impl CommandHandlerPort for CreateSafeUserCommand {
    type Context = Arc<dyn ContextPort>;
    type Output = ();

    async fn execute(&self, context: Self::Context) -> Result<Self::Output, Error> {
        let service = UserService::get_adapter(&context).await?;

        let bus = CqrsProvider::get_adapter(&context).await?;

        let check_user_query = GetUserByNameQuery::new(&self.name);

        let user = bus.query(Box::new(check_user_query)).await?;

        if user.is_some() {
            return Err("User already exists".into());
        }

        service
            .create_user(User::new(&self.name, &self.email))
            .await?;

        Ok(())
    }
}
