use crate::{context::AppContext, handlers};

pub fn router() -> axum::Router<AppContext> {
    axum::Router::new()
        .route("/", axum::routing::get(handlers::root_handler))
        .route(
            "/users",
            axum::routing::get(handlers::read_users).post(handlers::create_user),
        )
        .route(
            "/users/{user_id}",
            axum::routing::get(handlers::read_user)
                .put(handlers::update_user)
                .patch(handlers::partial_update_user)
                .delete(handlers::delete_user),
        )
}
