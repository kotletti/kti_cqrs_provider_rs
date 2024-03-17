mod services;

use std::{error::Error, sync::Arc};

use ioc_container_rs::context::{container_context::ContainerContext, context::Context};
use kti_cqrs_provider_rs::kti_cqrs_rs::core::bus::service_bus::ServiceBus;
use kti_cqrs_provider_rs::provider::cqrs_provider;
use services::user_service::{
  create_user_command::CreateUserCommand, get_user_by_name_query::GetUserByNameQuery,
  update_user_command::UpdateUserCommand, user_service::User,
};

pub struct UserController {
  context: Arc<ContainerContext>,
}

impl UserController {
  pub fn new(context: Arc<ContainerContext>) -> Self {
    Self { context }
  }

  pub fn token() -> &'static str {
    "USER_CONTROLLER"
  }

  pub async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn Error>> {
    let bus = self.get_cqrs_bus().await;

    let query = GetUserByNameQuery::new(name);

    bus.query(Box::new(query)).await
  }

  pub async fn create_user(&self, name: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let bus = self.get_cqrs_bus().await;

    let command = CreateUserCommand::new(name, email);

    bus.command(Box::new(command)).await;

    Ok(())
  }

  pub async fn update_user(&self, name: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let bus = self.get_cqrs_bus().await;

    let command = UpdateUserCommand::new(name, email);

    bus.command(Box::new(command)).await;

    Ok(())
  }

  async fn get_cqrs_bus(&self) -> cqrs_provider::Provider {
    self
      .context
      .resolve_provider::<cqrs_provider::Provider>(cqrs_provider::Provider::token())
      .await
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use ioc_container_rs::container::di::{InjectAdapter, DI};
  use kti_cqrs_provider_rs::di::create_cqrs_provider_di::create_cqrs_provider_di;
  use tokio::sync::Mutex;

  use crate::services::user_service::user_service::UserService;

  use super::*;

  fn get_users() -> Vec<User> {
    vec![
      User::new("Andrey", "andrey@mail.domain"),
      User::new("Daria", "daria@mail.domain"),
      User::new("Kirill", "kirill@mail.domain"),
    ]
  }

  async fn create_di() -> DI {
    let di = DI::new();

    let di = create_cqrs_provider_di(di).await;

    let store = Arc::new(Mutex::new(get_users()));

    let di = di
      .inject(InjectAdapter {
        token: UserService::token(),
        factory: Arc::new(move |_| UserService::new(store.clone())),
      })
      .await;

    let di = di
      .inject(InjectAdapter {
        token: UserController::token(),
        factory: Arc::new(|context| UserController::new(context)),
      })
      .await;

    di
  }

  #[tokio::test]
  async fn should_get_user_by_name() {
    let di = create_di().await;

    let context = di.get_context();

    let controller = context
      .resolve_provider::<UserController>(UserController::token())
      .await;

    let user_name = "Andrey";

    let user = match controller.get_user_by_name(&user_name).await.unwrap() {
      Some(user) => user,
      None => panic!("User not found."),
    };

    assert_eq!(user.get_name(), user_name);
  }

  #[tokio::test]
  async fn should_create_new_user() {
    let di = create_di().await;

    let context = di.get_context();

    let controller = context
      .resolve_provider::<UserController>(UserController::token())
      .await;

    let user_name = "Rita";

    let user_email = "rita@mail.domain";

    controller
      .create_user(&user_name, &user_email)
      .await
      .unwrap();

    let user = match controller.get_user_by_name(&user_name).await.unwrap() {
      Some(user) => user,
      None => panic!("User not found."),
    };

    assert_eq!(user.get_name(), user_name);
    assert_eq!(user.get_email(), user_email);
  }

  #[tokio::test]
  async fn should_update_user() {
    let di = create_di().await;

    let context = di.get_context();

    let controller = context
      .resolve_provider::<UserController>(UserController::token())
      .await;

    let user_name = "Andrey";

    let user_email = "andreyddk@mail.domain";

    controller
      .update_user(&user_name, &user_email)
      .await
      .unwrap();

    let user = match controller.get_user_by_name(&user_name).await.unwrap() {
      Some(user) => user,
      None => panic!("User not found."),
    };

    assert_eq!(user.get_name(), user_name);
    assert_eq!(user.get_email(), user_email);
  }
}
