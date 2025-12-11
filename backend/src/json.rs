use axum::{http::StatusCode, response::IntoResponse};
use facet::{Facet, ScalarType};
use facet_reflect::{HasFields, Peek};

pub struct Json<T>(pub T);

#[derive(Debug)]
pub struct JsonError(pub facet_json::JsonError);

impl IntoResponse for JsonError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::from(self).into_response()
    }
}

#[derive(Facet)]
pub struct MissingFields {
    missing_fields: Vec<&'static str>,
}

impl<'r, T: facet::Facet<'r>> Json<T> {
    pub fn iter_all_str_not_set(&'r self) -> impl Iterator<Item = &'static str> {
        let peek = Peek::new(&self.0);
        let st = peek.into_struct().expect("it is a struct");

        st.fields()
            .filter(|(field, _)| {
                tracing::trace!("{}", field.shape().ty);
                let shape = field.shape();

                shape
                    .scalar_type()
                    .is_some_and(|t| matches!(t, ScalarType::Str | ScalarType::String))
            })
            .filter(|(_, peek)| peek.as_str().is_some_and(|value| value.is_empty()))
            .map(|(field, _)| field.name)
    }

    pub fn collect_missing(&'r self) -> MissingFields {
        MissingFields {
            missing_fields: self.iter_all_str_not_set().collect(),
        }
    }

    pub fn is_all_str_set(&'r self) -> bool {
        self.iter_all_str_not_set().next().is_none()
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
