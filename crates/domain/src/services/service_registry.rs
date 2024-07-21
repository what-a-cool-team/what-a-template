use crate::services::greeting_service::{DomainGreetingService, DynGreetingService};
use std::sync::Arc;

#[derive(Clone)]
pub struct ServiceRegistry {
    pub greeting_service: DynGreetingService,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        let greeting_service = Arc::new(DomainGreetingService::new()) as DynGreetingService;
        ServiceRegistry { greeting_service }
    }
}
