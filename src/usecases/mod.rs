pub mod users {
    pub mod create_user {
        use std::sync::Arc;

        use axum::async_trait;
        use tokio::sync::Mutex;
        use tracing::{debug, info};

        use crate::{
            kernel::users::{
                create_user::{CreateUser, CreateUserError},
                User,
            },
            service::Service,
        };

        #[derive(Default)]
        pub struct CreateUserUseCase {
            next_id: Arc<Mutex<u64>>,
            users: Arc<Mutex<Vec<User>>>,
        }

        impl CreateUserUseCase {
            pub fn new() -> Self {
                Self {
                    next_id: Arc::new(Mutex::new(0)),
                    users: Arc::new(Mutex::new(Vec::new())),
                }
            }
        }

        #[async_trait]
        impl<'a> Service<&'a CreateUser> for CreateUserUseCase {
            type Response = User;
            type Error = CreateUserError;

            #[tracing::instrument(name = "create_user_usecase", skip(self))]
            async fn call(&self, req: &'a CreateUser) -> Result<Self::Response, Self::Error> {
                info!(
                    message = "checking if username already exists",
                    username = req.username,
                );

                let mut next_id = self.next_id.lock().await;

                let id = *next_id;

                *next_id += 1;

                let user = User {
                    id,
                    username: req.username.clone(),
                };

                let mut users = self.users.lock().await;

                debug!(
                    message = "savinng user",
                    username = req.username,
                );

                users.push(user.clone());

                Ok(user)
            }
        }
    }
}
