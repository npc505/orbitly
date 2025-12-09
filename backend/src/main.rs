use std::ops::Not;

use axum::{
    Router,
    body::Bytes,
    extract::{Request, State},
    http, middleware,
    response::{IntoResponse, Response},
};
use facet::Facet;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::json::Json;

mod args;
mod json;
mod neo4j;

#[derive(Clone)]
struct Ctx {
    neo4j: neo4rs::Graph,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = args::Args::parse();

    let ctx = Ctx {
        neo4j: neo4rs::Graph::connect(args.neo4j.to_config().expect("correct config"))
            .expect("failed to connect to neo4j instance"),
    };

    // Antes de iniciar ejecutamos todos los queries de constraint/schema/etc
    neo4j::Migrations::run(&ctx.neo4j)
        .await
        .expect("successful migrations");

    let auth = Router::new().route("/signup", axum::routing::post(register_user));

    let router = Router::new()
        .route("/ready", axum::routing::get(is_ready))
        .nest("/auth", auth)
        .layer(middleware::from_fn(log))
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind((args.address, args.port))
        .await
        .unwrap();

    tracing::info!(
        "Starting server on address {addr:?}",
        addr = listener.local_addr()
    );

    axum::serve(listener, router).await
}

#[derive(Facet, Debug, Clone, Copy)]
struct UserReq<'inp> {
    mail: &'inp str,
    #[facet(default)]
    first_name: Option<&'inp str>,
    #[facet(default)]
    last_name: Option<&'inp str>,
    username: &'inp str,
    password: &'inp str,
    password2: &'inp str,
}

async fn register_user(State(ctx): State<Ctx>, bytes: Bytes) -> Result<http::StatusCode, Response> {
    let json @ Json(user): Json<UserReq> =
        Json::from_bytes(&bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((
            http::StatusCode::BAD_REQUEST,
            http::HeaderMap::from_iter([(
                http::header::CONTENT_TYPE,
                http::HeaderValue::from_static("application/json"),
            )]),
            facet_json::to_string(&json.collect_missing()),
        )
            .into_response())?;
    }

    if user.password.trim() != user.password2.trim() {
        Err(http::StatusCode::BAD_REQUEST.into_response())?
    }

    ctx.neo4j
        .run(
            neo4rs::Query::new(String::from(
                r#"CREATE (u:User {
                    username: $username,
                    mail: $mail,
                    password: $password,
                    first_name: $first_name,
                    last_name: $last_name
                })"#,
            ))
            .param("username", user.username)
            .param("mail", user.mail)
            .param("password", password_auth::generate_hash(user.password))
            .param("first_name", user.first_name)
            .param("last_name", user.last_name),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    Ok(http::StatusCode::CREATED)
}

async fn log(req: Request, next: middleware::Next) -> Response {
    tracing::trace!("{method} {uri}", method = req.method(), uri = req.uri());
    next.run(req).await
}

async fn is_ready() -> Response {
    (http::StatusCode::OK, "ready").into_response()
}
