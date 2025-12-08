use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use f5a_services::om::UserDetail;
use tower::ServiceExt;

use crate::{setup::TestContext, test_ext::IntoValue, users::migrations};

#[tokio::test]
async fn test_read_user_detail() {
    let ctx = TestContext::new().await;
    ctx.setup_schema().await;

    migrations::insert_idesoft_user(ctx.db.as_ref())
        .await
        .unwrap();

    let app = ctx.configure();

    let req = Request::get("/api/users/1")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let value = res.into_value::<UserDetail>().await;
    assert_eq!(value.id, 1);
    assert_eq!(value.username, "idesoftd");
    assert_eq!(value.disabled, true);
}

#[tokio::test]
async fn it_returns_not_found_for_missing_user() {
    let ctx = TestContext::new().await;
    ctx.setup_schema().await;

    migrations::insert_idesoft_user(ctx.db.as_ref())
        .await
        .unwrap();

    let app = ctx.configure();

    let req = Request::get("/api/users/2")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
