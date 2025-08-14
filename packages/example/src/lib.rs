pub mod commands;
pub mod events;
pub mod queries;
pub mod services;

use std::sync::Arc;

use async_trait::async_trait;
use commands::create_safe_user_command::CreateSafeUserCommand;
use commands::create_user_command::CreateUserCommand;
use commands::update_user_command::UpdateUserCommand;
use events::rename_user_event::RenameUserEvent;
use ioc_container_rs::ports::{adapter_port::AdapterPort, context_port::ContextPort};
use kti_cqrs_provider_rs::kti_cqrs_rs::ports::bus::service_bus_port::ServiceBusPort;
use kti_cqrs_provider_rs::{
    kti_cqrs_rs::errors::error::Error, provider::cqrs_provider::CqrsProvider,
};
use queries::get_user_by_name_query::GetUserByNameQuery;
use services::user_service::User;

pub struct UserController {
    context: Arc<dyn ContextPort>,
}

#[async_trait]
impl AdapterPort<UserController> for UserController {
    fn token() -> &'static str {
        "UserController"
    }
}

impl UserController {
    pub fn new(context: Arc<dyn ContextPort>) -> Self {
        Self { context }
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, Error> {
        let bus = CqrsProvider::get_adapter(&self.context).await?;

        let query = GetUserByNameQuery::new(name);

        bus.query(Box::new(query)).await
    }

    pub async fn create_user(&self, name: &str, email: &str) -> Result<(), Error> {
        let bus = CqrsProvider::get_adapter(&self.context).await?;

        let command = CreateUserCommand::new(name, email);

        bus.command(Box::new(command)).await?;

        Ok(())
    }

    pub async fn create_safe_user(&self, name: &str, email: &str) -> Result<(), Error> {
        let bus = CqrsProvider::get_adapter(&self.context).await?;

        let command = CreateSafeUserCommand::new(name, email);

        bus.command(Box::new(command)).await?;

        Ok(())
    }

    pub async fn update_user_email(&self, name: &str, email: &str) -> Result<(), Error> {
        let bus = CqrsProvider::get_adapter(&self.context).await?;

        let command = UpdateUserCommand::new(name, email);

        bus.command(Box::new(command)).await?;

        Ok(())
    }

    pub async fn update_user_name(&self, current_name: &str, new_name: &str) -> Result<(), Error> {
        let bus = CqrsProvider::get_adapter(&self.context).await?;

        let event = RenameUserEvent::new(current_name, new_name);

        bus.event(Box::new(event)).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use ioc_container_rs::{
        container::di::{DI, InjectAdapter},
        context::container_context::ContainerContext,
    };
    use kti_cqrs_provider_rs::di::create_cqrs_provider_di::create_cqrs_provider_di;
    use services::user_service::UserService;
    use tokio::{sync::RwLock, time::sleep};

    use super::*;

    fn get_users() -> Vec<User> {
        vec![
            User::new("Andrey", "andrey@mail.domain"),
            User::new("Daria", "daria@mail.domain"),
            User::new("Kirill", "kirill@mail.domain"),
        ]
    }

    async fn create_di() -> Result<DI, Error> {
        let di = DI::new(Arc::new(ContainerContext::new()));

        let di = create_cqrs_provider_di(di).await?;

        let store = Arc::new(RwLock::new(get_users()));

        let di = di
            .inject(InjectAdapter {
                token: UserService::token(),
                factory: Arc::new(move |_| UserService::new(store.clone())),
            })
            .await?;

        let di = di
            .inject(InjectAdapter {
                token: UserController::token(),
                factory: Arc::new(|context| UserController::new(context)),
            })
            .await?;

        Ok(di)
    }

    #[tokio::test]
    async fn should_get_user_by_name() {
        let di = create_di().await.expect("Cant create DI");

        let context = di.get_context();

        let controller = UserController::get_adapter(&context)
            .await
            .expect("Cant resolve USER_CONTROLLER");

        let user_name = "Andrey";

        let user = controller
            .get_user_by_name(&user_name)
            .await
            .expect("Cant get user");

        assert!(user.is_some());

        let user = user.unwrap();

        assert_eq!(user.get_name(), user_name);
    }

    #[tokio::test]
    async fn should_create_new_user() {
        let di = create_di().await.expect("Cant create DI");

        let context = di.get_context();

        let controller = UserController::get_adapter(&context)
            .await
            .expect("Cant resolve USER_CONTROLLER");

        let user_name = "Rita";

        let user_email = "rita@mail.domain";

        controller
            .create_user(&user_name, &user_email)
            .await
            .expect("Cant create user");

        let user = controller
            .get_user_by_name(&user_name)
            .await
            .expect("Cant get user");

        assert!(user.is_some());

        let user = user.unwrap();

        assert_eq!(user.get_name(), user_name);
        assert_eq!(user.get_email(), user_email);
    }

    #[tokio::test]
    async fn should_be_created_safe_user() {
        let di = create_di().await.expect("Cant create DI");

        let context = di.get_context();

        let controller = UserController::get_adapter(&context)
            .await
            .expect("Cant resolve USER_CONTROLLER");

        let user_name = "Rita";
        let user_email = "rita@mail.domain";

        controller
            .create_safe_user(&user_name, &user_email)
            .await
            .expect("Cant create user");

        let user = controller
            .get_user_by_name(&user_name)
            .await
            .expect("Cant get user");

        assert!(user.is_some());

        let user = user.unwrap();

        assert_eq!(user.get_name(), user_name);
        assert_eq!(user.get_email(), user_email);
    }

    #[tokio::test]
    async fn should_be_not_created_safe_user() {
        let di = create_di().await.expect("Cant create DI");

        let context = di.get_context();

        let controller = UserController::get_adapter(&context)
            .await
            .expect("Cant resolve USER_CONTROLLER");

        let user_name = "Andrey";
        let user_email = "andrey@mail.domain";

        let user_creation = controller.create_safe_user(&user_name, &user_email).await;

        assert!(user_creation.is_err());
    }

    #[tokio::test]
    async fn should_update_user_email() {
        let di = create_di().await.expect("Cant create DI");

        let context = di.get_context();

        let controller = UserController::get_adapter(&context)
            .await
            .expect("Cant resolve USER_CONTROLLER");

        let user_name = "Andrey";

        let user_email = "andreyddk@mail.domain";

        controller
            .update_user_email(&user_name, &user_email)
            .await
            .expect("Cant update user");

        let user = controller
            .get_user_by_name(&user_name)
            .await
            .expect("Cant get user");

        assert!(user.is_some());

        let user = user.unwrap();

        assert_eq!(user.get_name(), user_name);
        assert_eq!(user.get_email(), user_email);
    }

    #[tokio::test]
    async fn should_update_user_name_by_event() {
        let di = create_di().await.expect("Cant create DI");

        let context = di.get_context();

        let controller = UserController::get_adapter(&context)
            .await
            .expect("Cant resolve USER_CONTROLLER");

        let current_user_name = "Daria";
        let new_user_name = "Rita";

        controller
            .update_user_name(&current_user_name, &new_user_name)
            .await
            .expect("Cant update user");

        sleep(Duration::from_secs(1)).await;

        let user = controller
            .get_user_by_name(&new_user_name)
            .await
            .expect("Cant get user");

        assert!(user.is_some());

        let user = user.unwrap();

        assert_eq!(user.get_name(), new_user_name);
    }
}
