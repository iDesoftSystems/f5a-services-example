use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use f5a_services::users::http::om::UserPage;
use tower::ServiceExt;

use crate::{setup::TestContext, test_ext::IntoValue, users::migrations};

#[tokio::test]
async fn it_reads_paginated_users() {
    let ctx = TestContext::new().await;
    ctx.setup_schema().await;

    migrations::insert_idesoft_user(ctx.db.as_ref())
        .await
        .unwrap();
    migrations::insert_blue_bird_user(ctx.db.as_ref())
        .await
        .unwrap();

    let app = ctx.configure();

    let req = Request::get("/api/users?page=0&page_size=10")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let value = res.into_value::<Vec<UserPage>>().await;
    assert_eq!(value.len(), 2);

    assert_eq!(value[0].username, "bluebird");
    assert_eq!(value[0].disabled, false);
    assert_eq!(value[1].username, "idesoftd");
    assert_eq!(value[1].disabled, true);
}

#[tokio::test]
async fn it_reads_empty_paginated_users() {
    let ctx = TestContext::new().await;
    ctx.setup_schema().await;

    let app = ctx.configure();

    let req = Request::get("/api/users?page=0&page_size=10")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let value = res.into_value::<Vec<UserPage>>().await;
    assert_eq!(value.len(), 0);
}
