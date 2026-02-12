use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{context::AppContext, root, users};

#[derive(OpenApi)]
#[openapi(
    info(description = "f5a services"),
    tags(
        (name = "user", description="User API endpoints")
    )
)]
pub struct ApiDoc;

pub fn router() -> axum::Router<AppContext> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(root::http::handlers::root_handler))
        .routes(routes!(
            users::http::handlers::read_users,
            users::http::handlers::create_user
        ))
        .routes(routes!(
            users::http::handlers::read_user,
            users::http::handlers::update_user,
            users::http::handlers::partial_update_user,
            users::http::handlers::delete_user
        ))
        .split_for_parts();

    router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
}
