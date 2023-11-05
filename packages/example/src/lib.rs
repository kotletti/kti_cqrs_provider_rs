mod services;

use std::error::Error;

use ioc_container_rs::context::{
  container_context::{ContainerContext, ContainerContextProps},
  context::Context,
};
use kti_cqrs_provider_rs::provider::CqrsProvider;
use kti_cqrs_rs::core::bus::ServiceBus;
use services::user_service::{
  create_user_command::CreateUserCommand, get_user_by_name_query::GetUserByNameQuery,
  update_user_command::UpdateUserCommand, user_service::User,
  user_service_context::UserServiceContext,
};

pub struct UserController {
  context: ContainerContext,
}

impl UserController {
  pub fn new(props: ContainerContextProps) -> Self {
    Self {
      context: ContainerContext::new(props),
    }
  }

  pub async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn Error>> {
    let bus = self.get_bus();

    let query = GetUserByNameQuery::new(name);

    bus.query(Box::new(query)).await
  }

  pub async fn create_user(&self, name: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let bus = self.get_bus();

    let command = CreateUserCommand::new(name, email);

    bus.command(Box::new(command)).await;

    Ok(())
  }

  pub async fn update_user(&self, name: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let bus = self.get_bus();

    let command = UpdateUserCommand::new(name, email);

    bus.command(Box::new(command)).await;

    Ok(())
  }

  fn get_bus(&self) -> Box<CqrsProvider::Provider<UserServiceContext>> {
    self.context.resolve_provider(CqrsProvider::TOKEN_PROVIDER)
  }
}

#[cfg(test)]
mod tests {
  use std::sync::{Arc, Mutex};

  use ioc_container_rs::container::Container;

  use crate::services::user_service::user_service::UserService;

  use super::*;

  fn get_users() -> Vec<User> {
    vec![
      User::new("Andrey", "andrey@mail.domain"),
      User::new("Daria", "daria@mail.domain"),
      User::new("Kirill", "kirill@mail.domain"),
    ]
  }

  #[tokio::test]
  async fn should_get_user_by_name() {
    let container = Container::new();

    let user_service = Arc::new(Mutex::new(UserService::new(get_users())));
    let user_service_context = Arc::new(Mutex::new(UserServiceContext::new(user_service)));

    container.register(CqrsProvider::TOKEN_PROVIDER, move || {
      Box::new(CqrsProvider::Provider::new(user_service_context.clone()))
    });

    let controller = UserController::new(ContainerContextProps {
      container,
      providers: vec![CqrsProvider::TOKEN_PROVIDER],
    });

    let user_name = "Andrey";

    let user = match controller.get_user_by_name(&user_name).await.unwrap() {
      Some(user) => user,
      None => panic!("User not found."),
    };

    assert_eq!(user.get_name(), user_name);
  }

  #[tokio::test]
  async fn should_create_new_user() {
    let container = Container::new();

    let user_service = Arc::new(Mutex::new(UserService::new(get_users())));
    let user_service_context = Arc::new(Mutex::new(UserServiceContext::new(user_service)));

    container.register(CqrsProvider::TOKEN_PROVIDER, move || {
      Box::new(CqrsProvider::Provider::new(user_service_context.clone()))
    });

    let controller = UserController::new(ContainerContextProps {
      container,
      providers: vec![CqrsProvider::TOKEN_PROVIDER],
    });

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
    let container = Container::new();

    let user_service = Arc::new(Mutex::new(UserService::new(get_users())));
    let user_service_context = Arc::new(Mutex::new(UserServiceContext::new(user_service)));

    container.register(CqrsProvider::TOKEN_PROVIDER, move || {
      Box::new(CqrsProvider::Provider::new(user_service_context.clone()))
    });

    let controller = UserController::new(ContainerContextProps {
      container,
      providers: vec![CqrsProvider::TOKEN_PROVIDER],
    });

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
