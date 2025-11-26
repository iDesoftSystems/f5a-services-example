use http_body_util::BodyExt;

pub trait IntoValue {
    fn into_value(self) -> impl Future<Output = serde_json::Value>;
}

impl IntoValue for axum::http::Response<axum::body::Body> {
    async fn into_value(self) -> serde_json::Value {
        let collected = self.into_body().collect().await.unwrap();
        let response_in_bytes = collected.to_bytes();

        serde_json::from_slice(&response_in_bytes).unwrap()
    }
}
