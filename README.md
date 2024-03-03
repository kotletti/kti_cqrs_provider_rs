# CQRS provider with ioc container

### Wrapped kti_cqrs_rs via provider for more complex usage

Simple example (existed in repo)

```
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

  fn get_bus(&self) -> Box<cqrs_provider::Provider<UserServiceContext>> {
    self.context.resolve_provider(cqrs_provider::TOKEN_PROVIDER)
  }
}
```
