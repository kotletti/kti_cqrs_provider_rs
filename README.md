# CQRS provider with ioc container

### Wrapped kti_cqrs_rs via provider for more complex usage

Simple example (existed in repo)

```rust
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
```
