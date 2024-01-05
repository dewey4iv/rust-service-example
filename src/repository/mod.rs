use crate::service::Service;

pub mod users {
    use super::*;

    pub mod insert_user {
        use crate::kernel::users::create_user::*;

        use super::*;

        pub trait InsertUserQuery:
            Service<CreateUser, Response = User, Error = CreateUserError>
        {
        }

        impl<T> InsertUserQuery for T where T: Service<CreateUser, Response = User, Error = CreateUserError> {}
    }

    pub mod list_users {
        use crate::kernel::users::{list_users::ListUsersError, User};

        use super::*;

        pub trait ListUsersQuery:
            Service<(), Response = Vec<User>, Error = ListUsersError>
        {
        }

        impl<T> ListUsersQuery for T where T: Service<(), Response = Vec<User>, Error = ListUsersError> {}
    }

    pub mod user_by_username {
        use crate::kernel::users::{user_by_username::UserByUsernameError, User};

        use super::*;

        pub trait UserByUsernameQuery:
            Service<(), Response = User, Error = UserByUsernameError>
        {
        }

        impl<T> UserByUsernameQuery for T where T: Service<(), Response = User, Error = UserByUsernameError> {}
    }
}
