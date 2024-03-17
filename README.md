# CQRS provider with ioc container

### Wrapped kti_cqrs_rs via provider for more complex usage

Simple example (existed in repo)

```rust
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
```
