use crate::setup;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn it_not_accept_empty_user_request() {
    let app = setup::configure().await;

    let req = Request::post("/users")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
