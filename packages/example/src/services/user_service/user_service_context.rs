use kti_cqrs_provider_rs::kti_cqrs_rs::common::context::Context;

use super::user_service::UserService;

#[derive(Clone)]
pub struct UserServiceContext {
  service: Box<UserService>,
}

impl UserServiceContext {
  pub fn new(service: Box<UserService>) -> Self {
    Self { service }
  }

  pub fn get_service(&self) -> &Box<UserService> {
    &self.service
  }
}

impl Context for UserServiceContext {}
