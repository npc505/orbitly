use std::{ops::Not, sync::Arc};

use axum::{
    Router,
    body::Bytes,
    extract::{Request, State},
    http, middleware,
    response::{IntoResponse, Response},
};
use facet::Facet;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{auth::Session, json::Json};

mod args;
mod auth;
mod json;
mod neo4j;

#[derive(Clone)]
struct Ctx {
    neo4j: neo4rs::Graph,
    sled_tree: Arc<Mutex<sled::Tree<1024>>>,
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
        sled_tree: Arc::new(Mutex::new(
            sled::open("/tmp/asdaksdj")
                .expect("failed to create")
                .open_tree("tokens")
                .expect("as"),
        )),
    };

    // Antes de iniciar ejecutamos todos los queries de constraint/schema/etc
    neo4j::Migrations::run(&ctx.neo4j)
        .await
        .expect("successful migrations");

    let protected = Router::new()
        .route("/category", axum::routing::post(create_category))
        .route("/category/search", axum::routing::post(search_category))
        .route("/genre", axum::routing::post(create_genre))
        .route("/genre/search", axum::routing::post(search_genre))
        .route(
            "/me/interest",
            axum::routing::get(get_interests)
                .post(like_interest)
                .delete(unlike_interest),
        )
        .route(
            "/me/match",
            axum::routing::post(perform_match).delete(unmatch),
        )
        .route("/me/matches", axum::routing::get(get_matches))
        .route("/me/lv2", axum::routing::get(get_lv2_matches))
        .route(
            "/me/recommendations",
            axum::routing::get(get_contenido_recomendado),
        )
        .route("/me/shortest-path", axum::routing::post(get_shortest_path))
        .route("/me", axum::routing::get(get_me))
        .route("/other", axum::routing::post(get_other_user))
        .route("/other/matches", axum::routing::post(get_other_user_matches))
        .route("/other/interest", axum::routing::post(get_other_user_interests))
        .route("/other/search", axum::routing::post(search_users))
        .route("/other/search/strict", axum::routing::post(search_users_strict))
        .route("/comunidades", axum::routing::get(comunidades))
        .route("/pagerank", axum::routing::get(page_rank))
        .layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::protect_routes,
        ));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/ready", axum::routing::get(async || "ready"))
        .nest("/auth", auth::router())
        .merge(protected)
        .layer(cors)
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
struct CategoryReq<'inp> {
    name: &'inp str,
    description: Option<&'inp str>,
}

#[derive(Facet, Debug, Clone, Copy)]
struct GenreReq<'inp> {
    name: &'inp str,
    description: Option<&'inp str>,
}

async fn create_genre(State(ctx): State<Ctx>, bytes: Bytes) -> Result<http::StatusCode, Response> {
    let json @ Json(genre): Json<GenreReq> =
        Json::from_bytes(&bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    let axum::Json(search_result) = search_impl(
        &ctx,
        SearchParams {
            term: genre.name,
            label: "Genre",
            cmp_field: "name",
            page: 0,
            page_size: 50,
        },
    )
    .await?;

    if search_result.name.is_empty() {
        ctx.neo4j
            .run(
                neo4rs::Query::new(String::from(
                    r#"CREATE (u:Genre {
                    name: $name,
                    description: $description
                })"#,
                ))
                .param("name", genre.name)
                .param("description", genre.description),
            )
            .await
            .map_err(neo4j::Error::from)
            .map_err(http::StatusCode::from)
            .map_err(|res| res.into_response())?;

        Ok(http::StatusCode::CREATED)
    } else {
        Ok(http::StatusCode::CONFLICT)
    }
}

#[derive(Facet, Debug, Clone, Copy)]
struct SearchReq<'inp> {
    term: &'inp str,
    #[facet(default)]
    page: Option<i64>,
    #[facet(default)]
    page_size: Option<i64>,
}

#[derive(Facet, serde::Serialize)]
struct SearchResponse {
    name: Vec<String>,
}

async fn search_category(
    State(ctx): State<Ctx>,
    bytes: Bytes,
) -> Result<axum::Json<SearchResponse>, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(category): Json<SearchReq> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    search_relaxed_impl(
        &ctx,
        SearchParams {
            term: category.term,
            label: "Category",
            cmp_field: "name",
            page: category.page.unwrap_or(0),
            page_size: category.page_size.unwrap_or(50),
        },
    )
    .await
}

async fn search_genre(
    State(ctx): State<Ctx>,
    bytes: Bytes,
) -> Result<axum::Json<SearchResponse>, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(search): Json<SearchReq> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    search_relaxed_impl(
        &ctx,
        SearchParams {
            term: search.term,
            label: "Genre",
            cmp_field: "name",
            page: search.page.unwrap_or(0),
            page_size: search.page_size.unwrap_or(50),
        },
    )
    .await
}

struct SearchParams<'inp> {
    term: &'inp str,
    label: &'inp str,
    cmp_field: &'inp str,
    page: i64,
    page_size: i64,
}

async fn search_impl<'inp>(
    ctx: &Ctx,
    SearchParams {
        term,
        label,
        cmp_field,
        ..
    }: SearchParams<'inp>,
) -> Result<axum::Json<SearchResponse>, Response> {
    let mut stream = ctx
        .neo4j
        .execute(
            neo4rs::Query::new(
                String::from(
                    r#"WITH $search AS rhs
                  MATCH (t:@LABEL)
                  WITH rhs, t,
                       apoc.text.clean(toLower(rhs)) AS normalizedCandidate,
                       apoc.text.clean(toLower(t.@CMP_FIELD)) AS normalizedExisting,
                       apoc.text.phonetic(rhs) AS candidatePhonetic,
                       apoc.text.phonetic(t.@CMP_FIELD) AS existingPhonetic,
                       apoc.text.distance(toLower(rhs), toLower(t.@CMP_FIELD)) AS distance

                  WHERE
                      toLower(rhs) = toLower(t.@CMP_FIELD)
                      OR
                      normalizedCandidate = normalizedExisting
                      OR
                      candidatePhonetic = existingPhonetic
                      OR
                      (distance <= 2 AND distance > 0)

                  RETURN
                      CASE
                          WHEN count(t) > 0 THEN false
                          ELSE true
                      END AS shouldCreateTag,
                      collect(DISTINCT t.@CMP_FIELD) AS existingAliases,
                      count(t) AS aliasCount,
                      rhs AS queriedTag"#,
                )
                .replace("@LABEL", label)
                .replace("@CMP_FIELD", cmp_field),
            )
            .param("search", term),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    if let Some(result) = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
    {
        let existing = result
            .get::<Vec<String>>("existingAliases")
            .unwrap_or_default();

        Ok(axum::Json(SearchResponse { name: existing }))
    } else {
        Ok(axum::Json(SearchResponse { name: vec![] }))
    }
}

async fn search_relaxed_impl<'inp>(
    ctx: &Ctx,
    SearchParams {
        term,
        label,
        cmp_field,
        page,
        page_size,
    }: SearchParams<'inp>,
) -> Result<axum::Json<SearchResponse>, Response> {
    let skip = page * page_size;

    let mut stream = ctx
        .neo4j
        .execute(
            neo4rs::Query::new(
                String::from(
                    r#"
                    MATCH (t:@LABEL)
                    WHERE toLower(t.@CMP_FIELD) CONTAINS toLower($search)
                    RETURN t.@CMP_FIELD AS name
                    SKIP $skip
                    LIMIT $limit
                    "#,
                )
                .replace("@LABEL", label)
                .replace("@CMP_FIELD", cmp_field),
            )
            .param("search", term)
            .param("skip", skip)
            .param("limit", page_size),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let mut names = vec![];
    while let Some(row) = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
    {
        if let Ok(name) = row.get::<String>("name") {
            names.push(name);
        }
    }

    Ok(axum::Json(SearchResponse { name: names }))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UserMatch {
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
    description: Option<String>,
    avatar: Option<String>,
    compatibility: f64,
}

#[derive(serde::Serialize)]
struct Lv2Response {
    matches: Vec<UserMatch>,
}

#[derive(serde::Serialize)]
struct CommunityMember {
    username: String,
    community_id: i64,
}

#[derive(serde::Serialize)]
struct ComunidadesResponse {
    communities: Vec<CommunityMember>,
}

async fn comunidades(
    State(ctx): State<Ctx>,
    _session: Session,
) -> Result<axum::Json<ComunidadesResponse>, Response> {
    let graph_name = format!("matchGraph_{}", rand::random::<u32>());

    ctx.neo4j
        .run(neo4rs::Query::new(format!(
            r#"
            MATCH (source:User)-[r:MATCHES]->(target:User)
            WITH gds.graph.project('{}', source, target) AS g
            RETURN g.graphName AS graph
            "#,
            graph_name
        )))
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let mut stream = ctx
        .neo4j
        .execute(neo4rs::Query::new(format!(
            r#"
            CALL gds.labelPropagation.stream('{}')
            YIELD nodeId, communityId AS Community
            RETURN gds.util.asNode(nodeId).username AS username, Community
            ORDER BY Community, username
            "#,
            graph_name
        )))
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let mut result = ComunidadesResponse {
        communities: vec![],
    };

    while let Some(row) = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
    {
        let username: String = row.get("username").unwrap_or_default();
        let community_id: i64 = row.get("Community").unwrap_or_default();
        result.communities.push(CommunityMember {
            username,
            community_id,
        });
    }

    let _ = ctx
        .neo4j
        .run(neo4rs::Query::new(format!(
            "CALL gds.graph.drop('{}', false)",
            graph_name
        )))
        .await;

    Ok(axum::Json(result))
}

#[derive(serde::Serialize)]
struct PageRankEntry {
    name: String,
    score: f64,
}

#[derive(serde::Serialize)]
struct PageRankResponse {
    rankings: Vec<PageRankEntry>,
}

async fn page_rank(
    State(ctx): State<Ctx>,
    _session: Session,
) -> Result<axum::Json<PageRankResponse>, Response> {
    let graph_name = format!("pageRankGraph_{}", rand::random::<u32>());

    let mut projection_stream = ctx.neo4j
        .execute(neo4rs::Query::new(format!(
            r#"
            CALL gds.graph.project(
                '{}',
                'Interest',
                '*'
            )
            "#,
            graph_name
        )))
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    while let Some(_) = projection_stream.next().await.map_err(neo4j::Error::from).map_err(http::StatusCode::from).map_err(|res| res.into_response())? {
    }

    let mut stream = ctx
        .neo4j
        .execute(neo4rs::Query::new(format!(
            r#"
            CALL gds.pageRank.stream('{}')
            YIELD nodeId, score
            RETURN gds.util.asNode(nodeId).name AS name, score
            ORDER BY score DESC, name ASC
            "#,
            graph_name
        )))
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let mut result = PageRankResponse { rankings: vec![] };

    while let Some(row) = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
    {
        let name: String = row.get("name").unwrap_or_default();
        let score: f64 = row.get("score").unwrap_or_default();
        result.rankings.push(PageRankEntry { name, score });
    }

    let _ = ctx
        .neo4j
        .run(neo4rs::Query::new(format!(
            "CALL gds.graph.drop('{}', false)",
            graph_name
        )))
        .await;

    Ok(axum::Json(result))
}

#[derive(Facet, serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Interest {
    name: String,
    #[facet(default)]
    description: Option<String>,
    #[facet(rename = "type")]
    #[facet(default)]
    #[serde(rename = "type")]
    kind: Option<String>,
}

#[derive(Facet, serde::Serialize, Debug, Clone)]
struct Interests {
    interests: Vec<Interest>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct User {
    first_name: Option<String>,
    last_name: Option<String>,
    mail: String,
    username: String,
}

async fn get_user_info_impl(
    ctx: &Ctx,
    current_username: &str,
    target_username: &str,
) -> Result<UserMatch, Response> {
    let mut stream = ctx
        .neo4j
        .execute_read(
            neo4rs::Query::new(String::from(
                r#"
                MATCH (u:User{username: $current_username})-[:LIKES]->(i1:Interest),
                      (other:User{username: $other_username})-[:LIKES]->(i2:Interest)
                WITH COLLECT(ID(i1)) AS u_likes, COLLECT(ID(i2)) AS other_likes, u, other
                WITH u, other, gds.similarity.cosine(u_likes, other_likes) AS compatibility
                RETURN
                    other.username as username,
                    other.first_name as first_name,
                    other.last_name as last_name,
                    other.description as description,
                    other.avatar as avatar,
                    compatibility
                "#,
            ))
            .param("current_username", current_username)
            .param("other_username", target_username),
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

    let Some(row) = row else {
        Err(http::StatusCode::NOT_FOUND.into_response())?
    };

    row.to::<UserMatch>().map_err(|err| {
        tracing::error!("Failed deserializing UserMatch {err}");
        http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })
}

async fn get_user_matches_impl(
    ctx: &Ctx,
    current_username: &str,
    target_username: &str,
) -> Result<Lv2Response, Response> {
    let mut stream = ctx
        .neo4j
        .execute_read(
            neo4rs::Query::new(String::from(
                r#"
                MATCH (u:User{username: $current_username})-[:LIKES]->(i1:Interest),
                      (other:User{username: $other_username})-[:MATCHES]->(m:User)-[:LIKES]->(i2:Interest)
                WITH COLLECT(ID(i1)) AS u_likes, COLLECT(ID(i2)) AS m_likes, u, m
                WITH u, m, gds.similarity.cosine(u_likes, m_likes) AS compatibility
                RETURN
                    m.username as username,
                    m.first_name as first_name,
                    m.last_name as last_name,
                    m.description as description,
                    m.avatar as avatar,
                    compatibility
                "#,
            ))
            .param("current_username", current_username)
            .param("other_username", target_username),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let mut result = Lv2Response { matches: vec![] };
    while let Some(row) = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
    {
        let user = row.to::<UserMatch>().map_err(|err| {
            tracing::error!("Failed deserializing UserMatch {err}");
            http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;
        result.matches.push(user);
    }

    Ok(result)
}

async fn get_me(State(ctx): State<Ctx>, session: Session) -> Result<axum::Json<UserMatch>, Response> {
    let user = get_user_info_impl(&ctx, &session.username, &session.username).await?;
    Ok(axum::Json(user))
}

#[derive(Facet, Debug, Clone, Copy)]
struct OtherUserParams<'inp> {
    username: &'inp str,
}

async fn get_other_user(
    State(ctx): State<Ctx>,
    session: Session,
    bytes: Bytes,
) -> Result<axum::Json<UserMatch>, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(params): Json<OtherUserParams> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    let user = get_user_info_impl(&ctx, &session.username, params.username).await?;
    Ok(axum::Json(user))
}

async fn get_matches(
    State(ctx): State<Ctx>,
    session: Session,
) -> Result<axum::Json<Lv2Response>, Response> {
    let result = get_user_matches_impl(&ctx, &session.username, &session.username).await?;
    Ok(axum::Json(result))
}

async fn get_other_user_matches(
    State(ctx): State<Ctx>,
    session: Session,
    bytes: Bytes,
) -> Result<axum::Json<Lv2Response>, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(params): Json<OtherUserParams> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    let result = get_user_matches_impl(&ctx, &session.username, params.username).await?;
    Ok(axum::Json(result))
}

async fn get_interests_impl(ctx: &Ctx, username: &str) -> Result<Interests, Response> {
    let mut stream = ctx
        .neo4j
        .execute_read(
            neo4rs::Query::new(String::from(
                r#"
                   MATCH (:User{username: $username})-[:LIKES]->(i:Interest) RETURN i
                "#,
            ))
            .param("username", username),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let mut result = Interests { interests: vec![] };
    while let Some(row) = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
    {
        let maybe_interest = row.to::<Interest>();
        match maybe_interest {
            Ok(row) => {
                result.interests.push(row);
            }
            Err(err) => {
                tracing::error!("Failed deserializing interest {err}");
                continue;
            }
        }
    }

    Ok(result)
}

async fn get_interests(
    State(ctx): State<Ctx>,
    session: Session,
) -> Result<axum::Json<Interests>, Response> {
    let result = get_interests_impl(&ctx, &session.username).await?;
    Ok(axum::Json(result))
}

async fn get_other_user_interests(
    State(ctx): State<Ctx>,
    _session: Session,
    bytes: Bytes,
) -> Result<axum::Json<Interests>, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(params): Json<OtherUserParams> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    let result = get_interests_impl(&ctx, params.username).await?;
    Ok(axum::Json(result))
}

async fn search_users(
    State(ctx): State<Ctx>,
    session: Session,
    bytes: Bytes,
) -> Result<axum::Json<Lv2Response>, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(search): Json<SearchReq> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    let page = search.page.unwrap_or(0);
    let page_size = search.page_size.unwrap_or(50);
    let skip = page * page_size;

    let mut stream = ctx
        .neo4j
        .execute(
            neo4rs::Query::new(String::from(
                r#"
                MATCH (u:User{username: $current_username})-[:LIKES]->(i1:Interest),
                      (other:User)-[:LIKES]->(i2:Interest)
                WHERE (toLower(other.username) CONTAINS toLower($term)
                   OR toLower(other.first_name) CONTAINS toLower($term)
                   OR toLower(other.last_name) CONTAINS toLower($term))
                  AND u <> other
                WITH COLLECT(ID(i1)) AS u_likes, COLLECT(ID(i2)) AS other_likes, u, other
                WITH u, other, gds.similarity.cosine(u_likes, other_likes) AS compatibility
                RETURN
                    other.username as username,
                    other.first_name as first_name,
                    other.last_name as last_name,
                    other.description as description,
                    other.avatar as avatar,
                    compatibility
                ORDER BY compatibility DESC
                SKIP $skip
                LIMIT $limit
                "#,
            ))
            .param("current_username", session.username)
            .param("term", search.term)
            .param("skip", skip)
            .param("limit", page_size),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let mut result = Lv2Response { matches: vec![] };
    while let Some(row) = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
    {
        let user = row.to::<UserMatch>().map_err(|err| {
            tracing::error!("Failed deserializing UserMatch {err}");
            http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;
        result.matches.push(user);
    }

    Ok(axum::Json(result))
}

async fn search_users_strict(
    State(ctx): State<Ctx>,
    bytes: Bytes,
) -> Result<axum::Json<SearchResponse>, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(search): Json<SearchReq> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    search_impl(
        &ctx,
        SearchParams {
            term: search.term,
            label: "User",
            cmp_field: "username",
            page: 0,
            page_size: 50,
        },
    )
    .await
}

#[derive(Facet, Debug, Clone, Copy)]
struct LikeInterestParams<'inp> {
    name: &'inp str,
}

async fn like_interest(
    State(ctx): State<Ctx>,
    session: Session,
    bytes: Bytes,
) -> Result<http::StatusCode, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(params): Json<LikeInterestParams> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    ctx.neo4j
        .run(
            neo4rs::Query::new(String::from(
                r#"
                MATCH (u:User {username: $username}), (i:Interest {name: $interest_name})
                MERGE (u)-[:LIKES]->(i)
                "#,
            ))
            .param("username", session.username)
            .param("interest_name", params.name),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    Ok(http::StatusCode::CREATED)
}

async fn unlike_interest(
    State(ctx): State<Ctx>,
    session: Session,
    bytes: Bytes,
) -> Result<http::StatusCode, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(params): Json<LikeInterestParams> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    ctx.neo4j
        .run(
            neo4rs::Query::new(String::from(
                r#"
                MATCH (u:User {username: $username})-[r:LIKES]->(i:Interest {name: $interest_name})
                DELETE r
                "#,
            ))
            .param("username", session.username)
            .param("interest_name", params.name),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    Ok(http::StatusCode::NO_CONTENT)
}

#[derive(Facet, Debug, Clone, Copy)]
struct ShortestPathParams<'inp> {
    target_label: &'inp str,
    target_name: &'inp str,
}

#[derive(serde::Serialize)]
struct ShortestPathResponse {
    path_length: i64,
    nodes: Vec<String>,
}

async fn get_shortest_path(
    State(ctx): State<Ctx>,
    session: Session,
    bytes: Bytes,
) -> Result<axum::Json<ShortestPathResponse>, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(params): Json<ShortestPathParams> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    let mut stream = ctx
        .neo4j
        .execute_read(
            neo4rs::Query::new(
                String::from(
                    r#"
                    MATCH (u1:User{username: $username}), (u2:@LABEL{name: $target_name})
                    MATCH p = shortestPath((u1)-[*..15]-(u2))
                    RETURN [n IN nodes(p) | coalesce(n.username, n.name)] AS path_nodes, length(p) AS path_length
                    "#,
                )
                .replace("@LABEL", params.target_label),
            )
            .param("username", session.username)
            .param("target_name", params.target_name),
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

    let Some(row) = row else {
        Err(http::StatusCode::NOT_FOUND.into_response())?
    };

    let path_length: i64 = row.get("path_length").unwrap_or_default();
    let nodes: Vec<String> = row.get("path_nodes").unwrap_or_default();

    Ok(axum::Json(ShortestPathResponse { path_length, nodes }))
}

#[derive(serde::Serialize)]
struct RecommendedContent {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<String>,
    score: f64,
    source: String,
}

#[derive(serde::Serialize)]
struct RecommendedContentResponse {
    recommendations: Vec<RecommendedContent>,
}

async fn get_contenido_recomendado(
    State(ctx): State<Ctx>,
    session: Session,
) -> Result<axum::Json<RecommendedContentResponse>, Response> {
    let mut stream = ctx
        .neo4j
        .execute_read(
            neo4rs::Query::new(String::from(
                r#"
                MATCH (u:User{username: $username})-[:LIKES]->(:Interest)<-[:LIKES]-(a:User)-[:LIKES]->(i:Interest)
                WHERE NOT (u)-[:LIKES]->(i)
                WITH DISTINCT a, i
                RETURN i.name AS name, i.description AS description, i.type AS type, toFloat(COUNT(a)) AS score, 'collaborative' AS source
                ORDER BY score DESC
                LIMIT 30
                "#,
            ))
            .param("username", session.username.as_str()),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let mut recommendations = vec![];
    while let Some(row) = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
    {
        let name: String = row.get("name").unwrap_or_default();
        let description: Option<String> = row.get("description").ok();
        let kind: Option<String> = row.get("type").ok();
        let score: f64 = row.get("score").unwrap_or(0.0);
        let source: String = row.get("source").unwrap_or_default();
        recommendations.push(RecommendedContent { name, description, kind, score, source });
    }

    Ok(axum::Json(RecommendedContentResponse {
        recommendations,
    }))
}

async fn get_lv2_matches(
    State(ctx): State<Ctx>,
    session: Session,
) -> Result<axum::Json<Lv2Response>, Response> {
    let mut stream = ctx
        .neo4j
        .execute_read(
            neo4rs::Query::new(String::from(
                r#"
                MATCH (u:User{username: $username})-[:LIKES]->(i1:Interest),
                      (lv2:User)-[:LIKES]->(i2:Interest)
                WHERE (u)-[:MATCHES]->(:User)-[:MATCHES]->(lv2)
                  AND NOT (u)-[:MATCHES]->(lv2)
                  AND u <> lv2
                WITH COLLECT(ID(i1)) AS u_likes, COLLECT(ID(i2)) AS lv2_likes, u, lv2
                WITH u, lv2, gds.similarity.cosine(u_likes, lv2_likes) AS compatibility
                RETURN
                    lv2.username as username,
                    lv2.first_name as first_name,
                    lv2.last_name as last_name,
                    lv2.description as description,
                    lv2.avatar as avatar,
                    compatibility
                ORDER BY compatibility DESC
                "#,
            ))
            .param("username", session.username),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let mut result = Lv2Response { matches: vec![] };
    while let Some(row) = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
    {
        let user = row.to::<UserMatch>().map_err(|err| {
            tracing::error!("Failed deserializing UserMatch {err}");
            http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;
        result.matches.push(user);
    }

    Ok(axum::Json(result))
}

#[derive(Facet, Debug, Clone, Copy)]
struct MatchParams<'inp> {
    target: &'inp str,
}

async fn perform_match(
    State(ctx): State<Ctx>,
    session: Session,
    bytes: Bytes,
) -> Result<http::StatusCode, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(match_params): Json<MatchParams> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    ctx.neo4j
        .run(
            neo4rs::Query::new(String::from(
                r#"
                    MATCH (u1:User { username: $username1 }), (u2:User { username: $username2 })
                    MERGE (u1)-[:MATCHES]->(u2)
            "#,
            ))
            .param("username1", session.username)
            .param("username2", match_params.target),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    Ok(http::StatusCode::CREATED)
}

async fn unmatch(
    State(ctx): State<Ctx>,
    session: Session,
    bytes: Bytes,
) -> Result<http::StatusCode, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(match_params): Json<MatchParams> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    ctx.neo4j
        .run(
            neo4rs::Query::new(String::from(
                r#"
                    MATCH (u1:User { username: $username1 })-[r:MATCHES]->(u2:User { username: $username2 })
                    DELETE r
            "#,
            ))
            .param("username1", session.username)
            .param("username2", match_params.target),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    Ok(http::StatusCode::NO_CONTENT)
}

async fn create_category(
    State(ctx): State<Ctx>,
    bytes: Bytes,
) -> Result<http::StatusCode, Response> {
    let bytes = bytes.iter().as_slice();
    let json @ Json(category): Json<CategoryReq> =
        Json::from_bytes(bytes).map_err(|err| err.into_response())?;

    if json.is_all_str_set().not() {
        Err((http::StatusCode::BAD_REQUEST).into_response())?;
    }

    let axum::Json(search_result) = search_impl(
        &ctx,
        SearchParams {
            term: category.name,
            label: "Category",
            cmp_field: "name",
            page: 0,
            page_size: 50,
        },
    )
    .await?;

    if search_result.name.is_empty().not() {
        ctx.neo4j
            .run(
                neo4rs::Query::new(String::from(
                    r#"CREATE (u:Category {
                    name: $name,
                    description: $description
                })"#,
                ))
                .param("name", category.name)
                .param("description", category.description),
            )
            .await
            .map_err(neo4j::Error::from)
            .map_err(http::StatusCode::from)
            .map_err(|res| res.into_response())?;

        Ok(http::StatusCode::CREATED)
    } else {
        Ok(http::StatusCode::CONFLICT)
    }
}

async fn log(req: Request, next: middleware::Next) -> Response {
    tracing::trace!("{method} {uri}", method = req.method(), uri = req.uri());
    next.run(req).await
}
