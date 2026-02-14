use crate::shared::context::AppContext;
use crate::shared::error::ApiError;
use crate::users::application::queries;
use crate::users::application::queries::ReadUserQuery;
use crate::users::http::om::UserDetail;
use axum::Json;
use axum::extract::{Path, State};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/users/{user_id}",
    tag = "user",
    params(
        ("user_id"=i32, Path, description = "User item unique id")
    ),
    responses(
        (status = OK, body = UserDetail),
        (status = NOT_FOUND, description = "User was not found"),
        (status = INTERNAL_SERVER_ERROR),
    )
)]
pub async fn read_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserDetail>, ApiError> {
    let query = ReadUserQuery { user_id };

    let user_detail = queries::ReadUserQueryHandler {
        conn: Arc::clone(&ctx.conn),
    }
    .handle(query)
    .await?;

    Ok(Json(user_detail))
}
