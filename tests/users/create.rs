use crate::setup::TestContext;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use sea_orm::EntityTrait;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn it_not_accept_empty_user_request() {
    let ctx = TestContext::new().await;
    let app = ctx.configure();

    let req = Request::post("/api/users")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn it_validate_required_user_params() {
    let ctx = TestContext::new().await;
    let app = ctx.configure();

    let user_params = json!({
        "name": "",
        "email": "",
        "username": "",
        "website": "",
        "age": 0,
        "password": "",
        "confirm_password": ""
    })
    .to_string();
    let req = Request::post("/api/users")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(user_params))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let expected_body = json!({
        "detail": "Validation failed",
        "errors": [
             {
                "code": "range",
                "field": "age",
                "reason": "Age must be between 18 and 100"
            },
             {
                "code": "email",
                "field": "email",
                "reason": "Email address is not valid"
            },
            {
                "code": "length",
                "field": "name",
                "reason": "name must be between 3 and 100 characters long"
            },
            {
                "code": "password_too_short",
                "field": "password",
                "reason": "Password must be at least 12 characters long"
            },
            {
                "code": "length",
                "field": "username",
                "reason": "Username must be between 3 and 100 characters long"
            },
            {
                "code": "url",
                "field": "website",
                "reason": "Website URL is not valid"
            }
        ],
    });
    let body_bytes = res.into_body().collect().await.unwrap().to_bytes();
    let body_content: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body_content, expected_body);
}

#[tokio::test]
async fn it_accepts_and_save_valid_user() {
    let ctx = TestContext::new().await;
    ctx.setup_schema().await;

    let app = ctx.configure();

    let user_params = json!({
        "name": "iDesoft Systems",
        "email": "idesoft@idesoft.co",
        "username": "idesoft",
        "website": "https://idesoft.co",
        "age": 18,
        "password": "iD3softSystems!",
        "confirm_password": "iD3softSystems!"
    })
    .to_string();
    let req = Request::post("/api/users")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(user_params))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let user_saved = schemas::user::Entity::find_by_id(1)
        .one(ctx.db.as_ref())
        .await
        .unwrap();
    assert!(user_saved.is_some());

    let user_model = user_saved.unwrap();
    assert_eq!(user_model.id, 1);
    assert_eq!(user_model.username, "idesoft");
    assert_eq!(user_model.creator_id, 1);
    assert!(user_model.disabled.is_positive());
}
