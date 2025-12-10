use std::ops::Not;

use axum::{
    RequestExt, Router,
    body::Bytes,
    extract::{FromRequestParts, OptionalFromRequestParts, Request, State},
    http,
    middleware::Next,
    response::{IntoResponse, Response},
};
use facet::Facet;

use crate::{Ctx, json::Json, neo4j};

#[derive(Facet, Clone, Copy)]
struct SigninReq<'inp> {
    username: &'inp str,
    password: &'inp str,
}

#[derive(Facet, serde::Serialize)]
struct Token {
    token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Session {
    token: String,
    #[serde(alias = "user")]
    pub username: String,
    created_on: jiff::Timestamp,
    duration: jiff::SignedDuration,
}

pub async fn protect_routes(state: State<Ctx>, mut req: Request, next: Next) -> Response {
    let session_extract_res = req
        .extract_parts_with_state::<Option<Session>, _>(&state)
        .await;

    if let Err(code) = session_extract_res {
        return code.into_response();
    }

    if session_extract_res
        .expect("we asserted it is an err")
        .is_none()
    {
        http::status::StatusCode::UNAUTHORIZED.into_response()
    } else {
        next.run(req).await
    }
}

impl FromRequestParts<Ctx> for Session {
    type Rejection = http::StatusCode;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &Ctx,
    ) -> Result<Self, Self::Rejection> {
        let axum_auth::AuthBearer(auth) = axum_auth::AuthBearer::from_request_parts(parts, state)
            .await
            .map_err(|_err| http::StatusCode::UNAUTHORIZED)?;
        let guard = state.sled_tree.lock().await;

        let session = {
            guard.get(&auth).map_err(|err| {
                tracing::error!("Failed reading key from sled tree {err:?}");
                http::StatusCode::INTERNAL_SERVER_ERROR
            })?
        };

        let session: Session = match session {
            Some(info) => serde_json::from_slice(&info).map_err(|err| {
                tracing::error!("Failed deserializing Session {err:?}");
                http::StatusCode::INTERNAL_SERVER_ERROR
            }),
            None => Err(http::StatusCode::UNAUTHORIZED),
        }?;

        if jiff::Timestamp::now()
            .duration_until(
                session
                    .created_on
                    .checked_add(session.duration)
                    .expect("in overflows we do not believe"),
            )
            .is_negative()
        {
            guard.remove(&auth).map_err(|err| {
                tracing::error!("Failed reading key from sled tree {err:?}");
                http::StatusCode::INTERNAL_SERVER_ERROR
            })?;

            Err(http::StatusCode::UNAUTHORIZED)?
        } else {
            Ok(session)
        }
    }
}

impl OptionalFromRequestParts<Ctx> for Session {
    type Rejection = http::StatusCode;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &Ctx,
    ) -> Result<Option<Self>, Self::Rejection> {
        let session = <Session as FromRequestParts<Ctx>>::from_request_parts(parts, state).await;
        match session {
            Ok(session) => Ok(Some(session)),
            Err(err) => match err {
                http::StatusCode::UNAUTHORIZED => Ok(None),
                other => Err(other),
            },
        }
    }
}

pub fn router() -> Router<Ctx> {
    Router::new()
        .route("/signin", axum::routing::post(login_user))
        .route("/signup", axum::routing::post(register_user))
}

impl Session {
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

fn generate_random_token<const LENGTH: usize>() -> String {
    use rand::Rng;

    rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(LENGTH)
        .map(char::from)
        .collect()
}

async fn login_user(State(ctx): State<Ctx>, bytes: Bytes) -> Result<axum::Json<Token>, Response> {
    let json @ Json(user): Json<SigninReq> =
        Json::from_bytes(&bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    let mut stream = ctx
        .neo4j
        .execute_read(
            neo4rs::Query::new(String::from(
                r#"MATCH (u:User {
                    username: $username
                }) RETURN u.password AS password"#,
            ))
            .param("username", user.username),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let row = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    if let Some(row) = row
        && password_auth::verify_password(
            user.password,
            row.get::<&str>("password").unwrap_or_default(),
        )
        .is_ok()
    {
        let guard = ctx.sled_tree.lock().await;
        let token = generate_random_token::<50>();

        guard
            .insert(
                &token,
                Session {
                    token: token.clone(),
                    username: user.username.to_string(),
                    created_on: jiff::Timestamp::now(),
                    duration: jiff::SignedDuration::from_mins(10),
                }
                .to_bytes(),
            )
            .unwrap();

        Ok(axum::Json(Token { token }))
    } else {
        Err(http::StatusCode::UNAUTHORIZED.into_response())
    }
}

#[derive(Facet, Debug, Clone, Copy)]
struct SignupParams<'inp> {
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
    let json @ Json(user): Json<SignupParams> =
        Json::from_bytes(&bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
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
