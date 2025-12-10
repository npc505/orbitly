use std::ops::Not;

use axum::http::StatusCode;

#[derive(rust_embed::Embed)]
#[folder = "../neo4j/cypher"]
#[include = "*.cypher"]
pub struct Migrations;

impl Migrations {
    pub async fn run(graph: &neo4rs::Graph) -> Result<(), neo4rs::Error> {
        for migration in Migrations::iter() {
            let span = tracing::trace_span!("migration", migration = migration.as_ref());
            tracing::debug!("Running {}", migration.as_ref());

            let _guard = span.enter();

            let contents = Migrations::get(&migration).expect("does exist");
            let str = std::str::from_utf8(&contents.data).expect("contains valid utf-8");
            let lines = str
                .lines()
                .map(|line| {
                    line.split_once("//")
                        .map(|(first, cmt)| {
                            tracing::trace!("Discarding comment {comment:?}", comment = cmt.trim());
                            first
                        })
                        .unwrap_or(line)
                        .trim()
                })
                .filter(|line| line.is_empty().not());

            let mut query = String::new();
            for line in lines {
                let rem = match line.split_once(";") {
                    Some((tail, head)) => {
                        query.push_str(tail);
                        query.push(';');
                        {
                            let query_str = query.trim();
                            tracing::trace!("Running query {query_str:?}");
                            graph.run(neo4rs::Query::new(query_str.to_string())).await?;

                            query.clear();
                        }

                        head
                    }
                    None => line,
                };

                query.push_str(rem);
                query.push(' ');
            }
        }

        Ok(())
    }
}

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

impl From<neo4rs::Error> for Error {
    fn from(err: neo4rs::Error) -> Self {
        tracing::error!("neo4j error {err:?}");
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
