pub mod users {
    use serde::{Deserialize, Serialize};

    use crate::service::Service;

    #[derive(Debug, Clone, Serialize)]
    pub struct User {
        pub id: u64,
        pub username: String,
    }

    pub mod create_user {
        use std::sync::Arc;

        pub use super::*;

        #[derive(Debug, Deserialize)]
        pub struct CreateUser {
            pub username: String,
        }

        #[derive(Debug, Serialize)]
        pub struct CreateUserError {
            pub reason: String,
        }

        pub trait CreateUserTrait<'a>:
            Service<&'a CreateUser, Response = User, Error = CreateUserError>
        {
        }

        impl<'a, T> CreateUserTrait<'a> for T where
            T: Service<&'a CreateUser, Response = User, Error = CreateUserError>
        {
        }

        pub type CreateUserService = Arc<dyn for<'a> CreateUserTrait<'a>>;
    }

    pub mod list_users {
        use std::sync::Arc;

        use super::*;

        #[derive(Debug, Serialize)]
        pub struct ListUsersError {}

        pub trait ListUsersTrait:
            Service<(), Response = Vec<User>, Error = ListUsersError>
        {
        }

        impl<T> ListUsersTrait for T where T: Service<(), Response = Vec<User>, Error = ListUsersError> {}

        pub type CreateUserService = Arc<dyn ListUsersTrait>;
    }

    pub mod user_by_username {
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        pub struct UserByUsernameError {}
    }
}
