use http_body_util::BodyExt;
use serde::de::DeserializeOwned;

pub trait IntoValue {
    fn into_value<T>(self) -> impl Future<Output = T>
    where
        T: DeserializeOwned;
}

impl IntoValue for axum::http::Response<axum::body::Body> {
    async fn into_value<T>(self) -> T
    where
        T: DeserializeOwned,
    {
        let collected = self.into_body().collect().await.unwrap();
        let response_in_bytes = collected.to_bytes();

        serde_json::from_slice(&response_in_bytes).unwrap()
    }
}
