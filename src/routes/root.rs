use crate::config::database::DatabaseContext;
use crate::libs::jwt::JWT;
use crate::middlewares::api_key_authorize_middleware::api_key_authorize_middleware;
use crate::routes::api_key;
use crate::routes::todo;
use axum::routing::IntoMakeService;
use axum::{middleware, Extension, Router};
use tower_http::trace::TraceLayer;

pub fn routes(db_context: DatabaseContext, jwt: JWT) -> IntoMakeService<Router> {
    let main_router: Router = { todo::routes() };

    let app_router = Router::new()
        .nest("/api", main_router.to_owned())
        .layer(middleware::from_fn_with_state(
            jwt.clone(),
            api_key_authorize_middleware,
        ))
        .nest("/api", api_key::routes())
        .layer(Extension(db_context.clone()))
        .layer(Extension(jwt.clone()))
        .layer(TraceLayer::new_for_http());

    app_router.into_make_service()
}
