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
        .route("/category/search", axum::routing::get(search_category))
        .route("/genre", axum::routing::post(create_genre))
        .route("/genre/search", axum::routing::get(search_genre))
        .route("/me/interest", axum::routing::get(get_interests))
        .route("/me/match", axum::routing::post(perform_match))
        .route("/me/lv2", axum::routing::get(get_lv2_matches))
        .route("/me/recommendations", axum::routing::get(get_contenido_recomendado))
        .route("/me/shortest-path", axum::routing::get(get_shortest_path))
        .route("/me", axum::routing::get(get_me))
        .route("/comunidades", axum::routing::get(comunidades))
        .route("/pagerank", axum::routing::get(page_rank))
        .layer(middleware::from_fn_with_state(
            ctx.clone(),
            auth::protect_routes,
        ));

    let router = Router::new()
        .route("/ready", axum::routing::get(async || "ready"))
        .nest("/auth", auth::router())
        .merge(protected)
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

    search_impl(
        &ctx,
        SearchParams {
            term: category.term,
            label: "Category",
            cmp_field: "name",
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

    search_impl(
        &ctx,
        SearchParams {
            term: search.term,
            label: "Genre",
            cmp_field: "name",
        },
    )
    .await
}

struct SearchParams<'inp> {
    term: &'inp str,
    label: &'inp str,
    cmp_field: &'inp str,
}

async fn search_impl<'inp>(
    ctx: &Ctx,
    SearchParams {
        term,
        label,
        cmp_field,
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
                       // Normalize both strings for comparison
                       apoc.text.clean(toLower(rhs)) AS normalizedCandidate,
                       apoc.text.clean(toLower(t.@CMP_FIELD)) AS normalizedExisting,
                       // Calculate phonetic similarity
                       apoc.text.phonetic(rhs) AS candidatePhonetic,
                       apoc.text.phonetic(t.@CMP_FIELD) AS existingPhonetic,
                       // Calculate string distance
                       apoc.text.distance(toLower(rhs), toLower(t.@CMP_FIELD)) AS distance

                  WHERE
                      // Case-insensitive exact match
                      toLower(rhs) = toLower(t.@CMP_FIELD)
                      OR
                      // Normalized match (handles spaces, special chars)
                      normalizedCandidate = normalizedExisting
                      OR
                      // Phonetic match (sounds alike)
                      candidatePhonetic = existingPhonetic
                      OR
                      // Very close string distance (handles typos like p0p vs pop)
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

#[derive(serde::Serialize)]
struct Lv2Response {
    matches: Vec<String>,
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

    let mut result = ComunidadesResponse { communities: vec![] };

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

    ctx.neo4j
        .run(neo4rs::Query::new(format!(
            r#"
            MATCH (source:User)-[:LIKES]->(:Interest)-[r:HAS_GENRE]->(target:Genre)
            WITH gds.graph.project('{}', source, target, {{relationshipProperties: r {{ .weight }}}}) AS g
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
    description: String,
    #[facet(rename = "type")]
    kind: String,
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

async fn get_me(State(ctx): State<Ctx>, session: Session) -> Result<axum::Json<User>, Response> {
    let mut stream = ctx
        .neo4j
        .execute_read(
            neo4rs::Query::new(String::from(
                r#"
                   MATCH (user:User{username: $username}) RETURN user   
                "#,
            ))
            .param("username", session.username),
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

    let Some(value) = row.map(|value| value.to::<User>().expect("failed")) else {
        Err(http::StatusCode::NOT_FOUND.into_response())?
    };

    Ok(axum::Json(value))
}

async fn get_interests(
    State(ctx): State<Ctx>,
    session: Session,
) -> Result<axum::Json<Interests>, Response> {
    let mut stream = ctx
        .neo4j
        .execute_read(
            neo4rs::Query::new(String::from(
                r#"
                   MATCH (:User{username: $username})-[:LIKES]->(i:Interest) RETURN i     
                "#,
            ))
            .param("username", session.username),
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

    Ok(axum::Json(result))
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
    count: i64,
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
                RETURN i.name AS name, COUNT(a) AS user_count
                ORDER BY user_count DESC
                "#,
            ))
            .param("username", session.username),
        )
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?;

    let mut result = RecommendedContentResponse {
        recommendations: vec![],
    };

    while let Some(row) = stream
        .next()
        .await
        .map_err(neo4j::Error::from)
        .map_err(http::StatusCode::from)
        .map_err(|res| res.into_response())?
    {
        let name: String = row.get("name").unwrap_or_default();
        let count: i64 = row.get("user_count").unwrap_or_default();
        result.recommendations.push(RecommendedContent { name, count });
    }

    Ok(axum::Json(result))
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
                     MATCH (u:User{username: $username})-[:MATCHES]->(:User)-[:MATCHES]->(lv2:User)
                     WHERE NOT (u)-[:MATCHES]->(lv2)
                    RETURN lv2.username as username
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
        let maybe_interest = row.get("username");
        match maybe_interest {
            Ok(row) => {
                result.matches.push(row);
            }
            Err(err) => {
                tracing::error!("Failed deserializing interest {err}");
                continue;
            }
        }
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
                    MATCH (u1:User { username: $username2 }), (u2:User { username: $username1 }) 
                    CREATE (u1)-[:MATCH]->(u2)
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
