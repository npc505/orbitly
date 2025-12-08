use axum::http::StatusCode;

pub enum Error {
    Other(neo4rs::Error),
    Client(ClientError),
}

pub enum ClientError {
    ContraintValidation,
}

impl From<Error> for StatusCode {
    fn from(value: Error) -> Self {
        match value {
            Error::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Client(client_err) => match client_err {
                ClientError::ContraintValidation => StatusCode::CONFLICT,
            },
        }
    }
}

// struct Neo4jErr(neo4rs::Error);

impl From<neo4rs::Error> for Error {
    fn from(err: neo4rs::Error) -> Self {
        match err {
            neo4rs::Error::Neo4j(ref neo4j_error) => {
                if let neo4rs::Neo4jErrorKind::Client(neo4rs::Neo4jClientErrorKind::Other) =
                    neo4j_error.kind()
                    && neo4j_error.code() == "Neo.ClientError.Schema.ConstraintValidationFailed"
                {
                    Error::Client(ClientError::ContraintValidation)
                } else {
                    Error::Other(err)
                }
            }
            err => Error::Other(err),
        }
    }
}
