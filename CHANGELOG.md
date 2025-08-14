# Changelog

## Version 0.3.1
* Update `ioc_container_rs` to v0.2.1
* Update `tokio` & `async-trait` deps
* Change tab size `2` -> `4`
* Remove impl `async fn get_adapter(...)` moved to `AdapterPort<T>`

## Version 0.3.0 BREAKING CHANGES
* Update `kti-cqrs-rs` to v0.3.0
* Add events
* Add error handling
* Implement `ServiceBusAdapter`
* Simplify tests & examples

## Version 0.2.0 BREAKING CHANGES
* Update `kti-cqrs-rs` to v0.2.0
* Modify tests
* Refactor provider

## Version 0.1.0 BREAKING CHANGES
* Replace the `std::sync::Mutex` to `tokio::sync::Mutex`
* Modify tests
* Add exports for `kti-cqrs-rs`

## Version 0.0.1
* Implement base logic commands & queries
* Add simple test cases
