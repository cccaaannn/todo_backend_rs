use crate::config::database::DatabaseContext;
use crate::routes::todo;
use axum::routing::IntoMakeService;
use axum::{middleware, Extension, Router};
use tower_http::trace::TraceLayer;

pub fn routes(db_context: DatabaseContext) -> IntoMakeService<Router> {
    let main_router = { todo::routes() };

    let app_router = Router::new()
        .nest("/api", main_router)
        .layer(Extension(db_context.clone()))
        .layer(TraceLayer::new_for_http());

    app_router.into_make_service()
}
