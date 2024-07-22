use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::repositories::greeting_repository::{DomainGreetingRepository, DynGreetingRepository};
use crate::services::greeting_service::{DomainGreetingService, DynGreetingService};

#[derive(Clone)]
pub struct ServiceRegistry {
    pub greeting_service: DynGreetingService,
}

impl ServiceRegistry {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let greeting_repository =
            Arc::new(DomainGreetingRepository::new(pool.clone())) as DynGreetingRepository;
        let greeting_service =
            Arc::new(DomainGreetingService::new(greeting_repository.clone())) as DynGreetingService;
        ServiceRegistry { greeting_service }
    }
}
