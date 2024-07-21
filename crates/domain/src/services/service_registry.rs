use std::sync::Arc;
use crate::services::greeting_service::{DynGreetingService, DomainGreetingService};

#[derive(Clone)]
pub struct ServiceRegistry {
    pub greeting_service: DynGreetingService,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        let greeting_service = Arc::new(DomainGreetingService::new()) as DynGreetingService;
        ServiceRegistry {
            greeting_service,
        }
    }
}
