use std::sync::{Arc, Mutex};

use kti_cqrs_rs::common::context::Context;

use super::user_service::UserService;

#[derive(Clone)]
pub struct UserServiceContext {
  service: Arc<Mutex<UserService>>,
}

impl UserServiceContext {
  pub fn new(service: Arc<Mutex<UserService>>) -> Self {
    Self { service }
  }

  pub fn get_service(&self) -> Arc<Mutex<UserService>> {
    self.service.clone()
  }
}

impl Context for UserServiceContext {}
