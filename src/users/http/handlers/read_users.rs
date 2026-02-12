use crate::context::AppContext;
use crate::error::ApiError;
use crate::pagination::Pagination;
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
    params(
        Pagination
    ),
    responses(
        (status=OK, body = [UserPage], description = "Get all users paginated"),
        (status = INTERNAL_SERVER_ERROR),
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
