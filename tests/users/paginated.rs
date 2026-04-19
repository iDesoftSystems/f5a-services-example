use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use f5a_services::users::http::om::UserPage;
use serde_json::json;
use tower::ServiceExt;

use crate::{
    setup::TestContext,
    test_ext::IntoValue,
    users::migrations::{insert_blue_bird_user, insert_chameleon_user, insert_idesoft_user},
};

#[tokio::test]
async fn it_reads_paginated_users() {
    let ctx = TestContext::new().await;
    ctx.setup_schema().await;

    insert_idesoft_user(ctx.db.as_ref()).await.unwrap();
    insert_blue_bird_user(ctx.db.as_ref()).await.unwrap();
    insert_chameleon_user(ctx.db.as_ref()).await.unwrap();

    // assert for page 0
    let app: axum::Router = ctx.configure();
    let req = Request::get("/api/users?page=0&page_size=1")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let value = res.into_value::<Vec<UserPage>>().await;
    assert_eq!(value.len(), 1);
    assert_eq!(value[0].username, "chameleon");
    assert_eq!(value[0].id, 3);
    assert_eq!(value[0].disabled, false);

    // assert for page 1
    let app: axum::Router = ctx.configure();
    let req = Request::get("/api/users?page=1&page_size=1")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let value = res.into_value::<Vec<UserPage>>().await;
    assert_eq!(value.len(), 1);
    assert_eq!(value[0].username, "bluebird");
    assert_eq!(value[0].id, 2);
    assert_eq!(value[0].disabled, false);

    // assert for page 2
    let app: axum::Router = ctx.configure();
    let req = Request::get("/api/users?page=2&page_size=1")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let value = res.into_value::<Vec<UserPage>>().await;
    assert_eq!(value.len(), 1);
    assert_eq!(value[0].username, "idesoft");
    assert_eq!(value[0].id, 1);
    assert_eq!(value[0].disabled, true);

    // assert for page 3
    let app: axum::Router = ctx.configure();
    let req = Request::get("/api/users?page=3&page_size=1")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let value = res.into_value::<Vec<UserPage>>().await;
    assert_eq!(value.len(), 0);
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

#[tokio::test]
async fn it_reads_paginated_users_with_idiomatic_json() {
    let ctx = TestContext::new().await;
    ctx.setup_schema().await;

    insert_idesoft_user(ctx.db.as_ref()).await.unwrap();

    let app: axum::Router = ctx.configure();
    let req = Request::get("/api/users?page=0&page_size=1")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let value = res.into_value::<serde_json::Value>().await;
    let expected_body = json!([
        {
            "id": 1,
            "username": "idesoft",
            "fullName": "iDesoft Systems",
            "disabled": 1,
            "createdAt": "2026-03-19T10:10:10Z"
        }
    ]);

    assert_eq!(value, expected_body);
}
