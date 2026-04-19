use crate::shared::context::AppContext;
use crate::shared::error::ApiError;
use crate::shared::pagination::Pagination;
use crate::users::application::queries;
use crate::users::application::queries::ReadUsersQuery;
use crate::users::http::om::UserPage;
use axum::Json;
use axum::extract::{Query, State};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "user",
    summary = "List all users (paginated)",
    description = r#"
## Use Case
Retrieves a paginated list of users from the system.
"#,
    params(
        Pagination
    ),
    responses(
        (status=OK, body = [UserPage], description = "Returns array of user summaries"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
    )
)]
pub async fn read_users(
    State(ctx): State<AppContext>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<UserPage>>, ApiError> {
    let query = ReadUsersQuery { pagination };
    let users_page = queries::ReadUsersQueryHandler {
        conn: Arc::clone(&ctx.conn),
    }
    .handle(query)
    .await?;

    Ok(Json(users_page))
}
