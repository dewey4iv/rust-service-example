use std::{
    any::{type_name, Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use tracing::info;

#[derive(Default, Clone)]
pub struct Provider {
    bindings: HashMap<TypeId, Arc<dyn Any + Send + Sync + 'static>>,
}

impl Provider {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn store<T: Any + Send + Sync + 'static>(&mut self, value: T) {
        self.bindings
            .insert(value.type_id(), Arc::new(value));
    }

    pub fn fetch<T: Any + Sized>(&self) -> &T {
        self.bindings
            .get(&TypeId::of::<T>())
            .and_then(|ptr| ptr.downcast_ref())
            .unwrap_or_else(|| {
                panic!(
                    "error getting {} from provider",
                    type_name::<T>()
                )
            })
    }
}

pub fn setup() -> Provider {
    info!("setting up provider");

    let mut provider = Provider::new();

    let _create_user = {
        use crate::kernel::users::create_user::CreateUserService;
        use crate::usecases::users::create_user::CreateUserUseCase;

        let create_user = Arc::new(CreateUserUseCase::new());
        provider.store::<CreateUserService>(create_user.clone());

        create_user
    };

    provider
}
