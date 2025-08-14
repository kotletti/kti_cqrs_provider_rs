use std::sync::Arc;

use ioc_container_rs::{
    container::di::{InjectAdapter, DI},
    ports::adapter_port::AdapterPort,
};
use kti_cqrs_rs::errors::error::Error;

use crate::provider::{
    command_bus_provider::CommandBusProvider, cqrs_provider::CqrsProvider,
    event_bus_provider::EventBusProvider, query_bus_provider::QueryBusProvider,
};

pub async fn create_cqrs_provider_di(di: DI) -> Result<DI, Error> {
    let di = di
        .inject(InjectAdapter {
            token: QueryBusProvider::token(),
            factory: Arc::new(|context| QueryBusProvider::new(context)),
        })
        .await?;

    let di = di
        .inject(InjectAdapter {
            token: CommandBusProvider::token(),
            factory: Arc::new(|context| CommandBusProvider::new(context)),
        })
        .await?;

    let di = di
        .inject(InjectAdapter {
            token: EventBusProvider::token(),
            factory: Arc::new(|context| EventBusProvider::new(context)),
        })
        .await?;

    let di = di
        .inject(InjectAdapter {
            token: CqrsProvider::token(),
            factory: Arc::new(|context| CqrsProvider::new(context)),
        })
        .await?;

    Ok(di)
}
