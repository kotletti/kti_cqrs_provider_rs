use std::sync::Arc;

use ioc_container_rs::container::di::{InjectAdapter, DI};
use kti_cqrs_rs::core::bus::{command_bus::CommandBus, query_bus::QueryBus};

use crate::provider::{
  command_bus_provider::CommandBusProvider, cqrs_provider, query_bus_provider::QueryBusProvider,
};

pub async fn create_cqrs_provider_di(di: DI) -> DI {
  let query_bus = Arc::new(QueryBus);

  di.inject(InjectAdapter {
    token: QueryBusProvider::token(),
    factory: Arc::new(move |context| QueryBusProvider::new(context, query_bus.clone())),
  })
  .await;

  let command_bus = Arc::new(CommandBus);

  di.inject(InjectAdapter {
    token: CommandBusProvider::token(),
    factory: Arc::new(move |context| CommandBusProvider::new(context, command_bus.clone())),
  })
  .await;

  let di = di
    .inject(InjectAdapter {
      token: cqrs_provider::Provider::token(),
      factory: Arc::new(|context| cqrs_provider::Provider::new(context)),
    })
    .await;

  di
}
