use std::ops::Not;

use axum::{
    Router,
    body::Bytes,
    extract::{Request, State},
    http, middleware,
    response::{IntoResponse, Response},
};
use facet::Facet;
use facet_pretty::FacetPretty;
use serde::Deserialize;
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

#[derive(Facet, Debug)]
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

#[derive(Deserialize, Debug)]
struct User<'inp> {
    mail: &'inp str,
    first_name: Option<&'inp str>,
    last_name: Option<&'inp str>,
    username: &'inp str,
}

async fn register_user(State(ctx): State<Ctx>, bytes: Bytes) -> Result<(), Response> {
    let json @ Json(
        user @ UserReq {
            mail,
            first_name,
            last_name,
            username,
            password,
            password2,
        },
    ) = &Json::from_bytes(&bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    tracing::info!(
        "{}",
        user.pretty_with(
            facet_pretty::PrettyPrinter::new()
                .with_colors(false)
                .with_indent_size(4)
        )
    );

    if password.trim() != password2.trim() {
        Err(http::StatusCode::BAD_REQUEST.into_response())?
    }

    let hash = password_auth::generate_hash(password);

    let res = ctx
        .neo4j
        .execute(
            neo4rs::Query::new(
                "CREATE (u:User { username: $username, mail: $mail, password: $password }) RETURN u"
                    .to_string(),
            )
            .param("username", *username)
            .param("mail", *mail)
            .param("password", hash),
        )
        .await;

    let res = res
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
        .next()
        .await;

    tracing::trace!("{:?}", res);

    let row = res
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
        .unwrap();

    let user = row.to::<User>();

    tracing::trace!("{:?}", user);

    Ok(())
}

async fn log(req: Request, next: middleware::Next) -> Response {
    tracing::debug!("{method} {uri}", method = req.method(), uri = req.uri());
    next.run(req).await
}

async fn is_ready() -> Response {
    (http::StatusCode::OK, "ready").into_response()
}
