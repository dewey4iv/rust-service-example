use std::error::Error;
use std::net::SocketAddr;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;

use crate::provider::Provider;

pub async fn start(addr: &SocketAddr, provider: impl Into<Provider>) -> Result<(), Box<dyn Error>> {
    let tcp_listener = TcpListener::bind(addr).await?;
    let provider = provider.into();

    let router = Router::new()
        .route("/", get(index))
        .nest("/users", users::router())
        .with_state(provider);

    axum::serve(tcp_listener, router.into_make_service()).await?;

    Ok(())
}

async fn index() -> impl IntoResponse {
    ""
}

pub mod users {
    use crate::provider::Provider;

    use super::*;

    pub fn router() -> Router<Provider> {
        Router::new().route("/", post(create_user::handler))
    }

    pub mod create_user {
        use axum::Json;
        use serde::{Deserialize, Serialize};

        use crate::{kernel::users::create_user::*, response::Response};

        use super::*;

        #[derive(Debug, Deserialize)]
        pub struct CreateUserReq {
            pub user: CreateUser,
        }

        #[derive(Debug, Serialize)]
        pub struct CreateUserRes {
            pub user: User,
        }

        #[tracing::instrument(name = "create_user_http", skip(provider))]
        pub async fn handler(
            State(provider): State<Provider>,
            Json(params): Json<CreateUserReq>,
        ) -> impl IntoResponse {
            let service = provider.fetch::<CreateUserService>();

            let result = service.call(&params.user).await;

            match result {
                Ok(user) => Response::success()
                    .payload(CreateUserRes { user })
                    .notice("user successfully created!"),
                Err(err) => Response::fail().error(err),
            }
        }
    }
}
