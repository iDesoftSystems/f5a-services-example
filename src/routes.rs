use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{root, shared::context::AppContext, users};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "F5A Services API",
        description = r#"
## Overview
REST API for F5A.

## Authentication
Currently no authentication required (internal service).
        "#,
        version = "1.0.0"
    ),
    components(
        schemas(
            crate::shared::pagination::Pagination,
            crate::shared::response::ProblemDetails,
            crate::shared::response::Field,
            crate::users::http::om::CreateUserParams,
            crate::users::http::om::UserCreated,
            crate::users::http::om::UserPage,
            crate::users::http::om::UserDetail,
            crate::users::http::om::UpdateUserParams,
            crate::users::http::om::PartialUserParams,
        )
    ),
    tags(
        (name = "user", description = "User management operations: create, read, update, delete users")
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
