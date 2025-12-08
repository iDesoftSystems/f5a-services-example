use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use sea_orm::EntityTrait;
use serde_json::{Value, json};
use tower::ServiceExt;

use crate::{setup::TestContext, test_ext::IntoValue, users::migrations::insert_idesoft_user};

fn update_user_url(user_id: i32) -> String {
    format!("/api/users/{user_id}")
}

#[tokio::test]
async fn it_not_accept_empty_user_request() {
    let ctx = TestContext::new().await;
    let app = ctx.configure();

    let req = Request::put(update_user_url(1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn it_validate_required_user_fields_to_update() {
    let ctx = TestContext::new().await;
    let app = ctx.configure();

    let update_user_params = json!({
        "username": "",
        "disabled": false,
    });

    let req = Request::put(update_user_url(1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(
            serde_json::to_string(&update_user_params).unwrap(),
        ))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let expected_body = json!({
        "errors": [
            {
                "code": "length",
                "field": "username",
                "reason": "The username must be between 3 and 100 characters"
            }
        ],
        "detail": "Validation failed"
    });
    assert_eq!(res.into_value::<Value>().await, expected_body)
}

#[tokio::test]
async fn it_not_accept_unknown_user_id() {
    let ctx = TestContext::new().await;
    ctx.setup_schema().await;

    let app = ctx.configure();

    let update_user_params = json!({
        "username": "idesoftd",
        "disabled": false,
    });

    let req = Request::put(update_user_url(10))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(
            serde_json::to_string(&update_user_params).unwrap(),
        ))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn it_accept_and_update_user() {
    let ctx = TestContext::new().await;
    ctx.setup_schema().await;

    let user_id_to_update = 1;

    insert_idesoft_user(ctx.db.as_ref()).await.unwrap();

    let app = ctx.configure();

    let update_user_params = json!({
        "username": "idesoftd-mod",
        "disabled": true,
    });

    let req = Request::put(update_user_url(user_id_to_update))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(
            serde_json::to_string(&update_user_params).unwrap(),
        ))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let updated_person = schemas::user::Entity::find_by_id(user_id_to_update)
        .one(ctx.db.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_person.username, "idesoftd-mod");
    assert_eq!(updated_person.disabled, 1);
}
