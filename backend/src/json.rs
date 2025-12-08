use std::ops::Not;

use axum::{http::StatusCode, response::IntoResponse};
use facet::ScalarType;
use facet_reflect::{HasFields, Peek};

pub struct Json<T>(pub T);

#[derive(Debug)]
pub struct JsonError(pub facet_json::JsonError);

impl IntoResponse for JsonError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::from(self).into_response()
    }
}

impl<'r, T: facet::Facet<'r>> Json<T> {
    pub fn is_all_str_set(&'r self) -> bool {
        let peek = Peek::new(&self.0);
        let st = peek.into_struct().expect("it is a struct");

        st.fields()
            .filter_map(|(field, peek)| {
                tracing::trace!("{}", field.shape().ty);
                let shape = field.shape();

                shape
                    .scalar_type()
                    .is_some_and(|t| matches!(t, ScalarType::Str | ScalarType::String))
                    .then_some(peek)
            })
            .all(|peek| peek.as_str().is_some_and(|value| value.is_empty().not()))
    }
}

impl From<JsonError> for StatusCode {
    fn from(_value: JsonError) -> Self {
        StatusCode::BAD_REQUEST
    }
}

impl<'req, T: facet::Facet<'req>> Json<T> {
    pub fn from_bytes(value: &'req [u8]) -> Result<Self, JsonError> {
        facet_json::from_slice_borrowed::<T>(value)
            .map(Json)
            .map_err(|err| {
                let err = if let Ok(source_str) = str::from_utf8(value) {
                    err.with_source(source_str)
                } else {
                    err
                };

                tracing::error!(
                    "Failed deserializing value {} ({:?}) {}",
                    err.kind,
                    err.span.unwrap_or_default(),
                    err.source_code.as_deref().unwrap_or("no source")
                );

                JsonError(err)
            })
    }
}

/* impl<T: for<'a> facet::Facet<'a>, S: Sync> FromRequest<S> for Json<T> {
    type Rejection = axum::http::StatusCode;

    async fn from_request(
        req: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let method = req.method().clone();
        let uri = req.uri().clone();
        let maybe_bytes = axum::body::to_bytes(req.into_body(), 20971520).await;

        let bytes = match maybe_bytes {
            Ok(bytes) => bytes,
            Err(err) => {
                tracing::error!(
                    "{method} {uri} Error extracting body bytes {err:?}",
                    method = method,
                    uri = uri
                );
                Err(axum::http::StatusCode::FORBIDDEN)?
            }
        };

        let maybe_t: Result<T, _> = facet_json::from_slice(&bytes);
        match maybe_t {
            Ok(t) => Ok(Json(t)),
            Err(err) => {
                tracing::error!(
                    "{method} {uri} Error deserializing body {err:?}",
                    method = method,
                    uri = uri
                );
                Err(axum::http::StatusCode::BAD_REQUEST)?
            }
        }
    }
} */
