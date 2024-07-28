use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_auth::AuthBearer;
use chrono::{DateTime, Duration, Utc};
use futures::{TryFutureExt, TryStreamExt};
use jsonwebtoken::{
    decode, encode,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::{postgres::PgPoolOptions, Executor, Postgres};
use std::{os::macos::raw::stat, sync::Arc};
use tokio::main;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    exp: isize,
    nonce: usize,
}
#[derive(Debug, Serialize, Deserialize)]
struct AppleClaims {
    sub: String,
    aud: String,
    exp: isize,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct AppleIDSignInResponse {
    server_jwt: String,
    refresh_token: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct RefreshTokenRequest {
    refresh_token: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct RefreshTokenResponse {
    server_jwk_token: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct LogInWithJwt {
    jwt: String,
    familyName: Option<String>,
    givenName: Option<String>,
    email: Option<String>,
    user: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Event {
    id: i32,
    title: String,
    desc: String,
    public: bool,
}
#[derive(Deserialize, Debug)]
struct CreateEventPost {
    event: Event,
}
#[derive(Serialize)]
struct DiscoverResponse {
    resp: Vec<Event>,
}
async fn verify_apple_jwk(token: String) -> (bool, Option<TokenData<AppleClaims>>) {
    let jwk_set = reqwest::get("https://appleid.apple.com/auth/keys")
        .await
        .unwrap()
        .json::<JwkSet>()
        .await
        .unwrap();
        for jwk in jwk_set.keys {
            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_audience(&["com.mast3rsoft.MeetlyIOSApp"]);
            if let Ok(token) = decode::<AppleClaims>(
                &token,
                &DecodingKey::from_jwk(&jwk).unwrap(),
                &validation,
            ) {
                info!("key found");
                return (true, Some(token));
            }
        
    }
    (false, None)
}

async fn verify_refresh_jwk(token: String) -> (bool, Option<TokenData<Claims>>) {
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_audience(&["com.mast3rsoft.MeetlyApp.Refresh"]);
    let key = DecodingKey::from_ec_pem(include_bytes!("../meetly-jwt-sig.pem.pub")).unwrap();
    if let Ok(token) = decode::<Claims>(&token, &key, &validation) {
        return (true, Some(token));
    }
    (false, None)
}
async fn verify_server_jwk(token: String) -> (bool, Option<TokenData<Claims>>) {
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_audience(&["com.mast3rsoft.MeetlyApp.Identity"]);
    let key = DecodingKey::from_ec_pem(include_bytes!("../meetly-jwt-sig.pem.pub")).unwrap();
    if let Ok(token) = decode::<Claims>(&token, &key, &validation) {
        return (true, Some(token));
    }
    (false, None)
}
async fn create_event(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    AuthBearer(token): AuthBearer,
    State(state): State<Arc<AppState>>,
    axum::Json(payload): axum::Json<CreateEventPost>,
) -> StatusCode {
    // insert your application logic here
    if !verify_server_jwk(token).await.0 {
        return StatusCode::UNAUTHORIZED;
    }
    let mut execute_query: sqlx::query::Query<Postgres, sqlx::postgres::PgArguments> = sqlx::query(r#"INSERT INTO "public"."events"("title", "desc", "public") VALUES($1, $2, $3) RETURNING "id", "title", "desc", "public";"#).bind(payload.event.title).bind(payload.event.desc).bind(payload.event.public);
    // this will be converted into a JSON response\
    state.con.fetch(execute_query).try_next().await.unwrap();
    // with a status code of `201 Created`
    StatusCode::CREATED
}

/// refreshes their server token
async fn refresh_token_path(
    State(state): State<Arc<AppState>>,
    axum::Json(payload): axum::Json<RefreshTokenRequest>,
    //first verify with JWT
    // create_user_if needed
) -> (StatusCode, axum::Json<RefreshTokenResponse>) {
    let tk = verify_refresh_jwk(payload.refresh_token).await;
    if !tk.0 {
        // token invalid
        return (
            StatusCode::UNAUTHORIZED,
            axum::Json::from(RefreshTokenResponse::default()),
        );
    }
    let mut date = Utc::now();
    date += Duration::days(90);
    let trk = tk.1.unwrap();
    let server_jwt_claim = Claims {
        sub: trk.claims.sub,
        aud: String::from("com.mast3rsoft.MeetlyApp.Identity"),
        exp: date.timestamp() as isize,
        nonce: rand::random(),
    };
    let server_jwt_token = encode(
        &Header::new(Algorithm::ES256),
        &server_jwt_claim,
        &EncodingKey::from_ec_pem(include_bytes!("../meetly-jwt-sig.pem")).unwrap(),
    )
    .unwrap();
    return (
        StatusCode::CREATED,
        axum::Json::from(RefreshTokenResponse {
            server_jwk_token: server_jwt_token,
        }),
    );
}

/// This api path creates a user, or refreshes their refresh token if lost
async fn signin_apple(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<Arc<AppState>>,
    axum::Json(payload): axum::Json<LogInWithJwt>,
    //first verify with JWT
    // create_user_if needed
) -> (StatusCode, axum::Json<AppleIDSignInResponse>) {
    info!("Sign in request!");
    // check if this a legit apple request
    if !verify_apple_jwk(payload.jwt).await.0 {
        warn!("Invalid JWT");
        return (StatusCode::UNAUTHORIZED, axum::Json::default()); // TODO: Fix perf of this..
    }
    // generate JWTs
    let mut checkExistingEvent_query: sqlx::query::Query<Postgres, sqlx::postgres::PgArguments> =
        sqlx::query("SELECT * FROM users WHERE useridapple = '$1'").bind(&payload.user);
    // insert user data
    let mut userAlreadyPresent = false;
    if let Ok(doc) = state.con.fetch(checkExistingEvent_query).try_next().await {
        if let Some(_) = doc {
            userAlreadyPresent = true;
        }
    }
    if !userAlreadyPresent {
        let insert_query: sqlx::query::Query<Postgres, sqlx::postgres::PgArguments> = sqlx::query(r#"INSERT INTO "public"."users"("familyName", "givenName", "useridapple", "email") VALUES($1, $2, $3, $4) RETURNING "id", "familyName", "givenName", "useridapple","email";"#).bind(payload.familyName).bind(payload.givenName).bind(&payload.user).bind(payload.email);
        state.con.fetch(insert_query).try_next().await.unwrap(); // insert the users
    }
    // created tokens..
    // TODO: Implement rotating refresh tokens........
    let refresh_token_claim: Claims = Claims {
        sub: payload.user.clone(),
        aud: String::from("com.mast3rsoft.MeetlyApp.Refresh"),
        exp: u32::MAX as isize,
        nonce: rand::random(),
    };
    let mut date = Utc::now();
    date += Duration::days(90);
    let server_jwt_claim = Claims {
        sub: payload.user,
        aud: String::from("com.mast3rsoft.MeetlyApp.Identity"),
        exp: date.timestamp() as isize,
        nonce: rand::random(),
    };
    let refresh_token = encode(
        &Header::new(Algorithm::ES256),
        &refresh_token_claim,
        &EncodingKey::from_ec_pem(include_bytes!("../meetly-jwt-sig.pem")).unwrap(),
    )
    .unwrap();
    let server_jwt_token = encode(
        &Header::new(Algorithm::ES256),
        &server_jwt_claim,
        &EncodingKey::from_ec_pem(include_bytes!("../meetly-jwt-sig.pem")).unwrap(),
    )
    .unwrap();
    return (
        StatusCode::CREATED,
        axum::Json::from(AppleIDSignInResponse {
            server_jwt: server_jwt_token,
            refresh_token: refresh_token,
        }),
    );
}
#[derive(Clone)]
struct AppState {
    con: sqlx::Pool<Postgres>,
}
async fn discover_resp(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, axum::Json<DiscoverResponse>) {
    // insert your application logic here

    let mut fetch_query: sqlx::query::Query<Postgres, sqlx::postgres::PgArguments> =
        sqlx::query("SELECT * FROM events WHERE public = $1").bind(true);

    let mut rows = state.con.fetch(fetch_query);
    let mut events = vec![];
    while let Some(row) = rows.try_next().unwrap_or_else(|_| panic!("NIGG!")).await {
        // map the row into a user-defined domain type
        let title: &str = row.try_get("title").unwrap();
        let desc: &str = row.try_get("desc").unwrap();
        let id: i32 = row.try_get("id").unwrap();

        events.push(Event {
            title: String::from(title),
            desc: String::from(desc),
            id,
            public: true, // if this is wrong then KABOOM!
        })
    }
    let discover_resp = DiscoverResponse { resp: events };
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::FOUND, axum::Json(discover_resp))
}
#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    // build our application with a single route
    let con = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:12345678@localhost/postgres")
        .await;
    if let Err(nig) = con {
        error!("Error while connecting");
        std::process::exit(1);
    }
    let con = con.unwrap();
    info!("DB Connected!");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/discover_events", get(discover_resp))
        .route("/refresh_token", post(refresh_token_path))
        .route("/signin_apple", post(signin_apple))
        .route("/create_event", post(create_event))
        .with_state(AppState { con }.into());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Starting server at 0.0.0.0:3000");
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    info!("Connecting to DB");

    axum::serve(listener, app).await.unwrap();
}
