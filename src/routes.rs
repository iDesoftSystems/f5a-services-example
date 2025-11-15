use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{context::AppContext, handlers};

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
        .routes(routes!(handlers::root::root_handler))
        .routes(routes!(
            handlers::users::read_users,
            handlers::users::create_user
        ))
        .routes(routes!(
            handlers::users::read_user,
            handlers::users::update_user,
            handlers::users::partial_update_user,
            handlers::users::delete_user
        ))
        .split_for_parts();

    router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
}
