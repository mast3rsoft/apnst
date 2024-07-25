use std::{os::macos::raw::stat, sync::Arc};

use axum::{
    extract::State, http::StatusCode, routing::{get, post}, Router
};
use sqlx::Row;
use tracing::{info,error, Level};
use tracing_subscriber::FmtSubscriber;
use tokio::main;
use sqlx::{postgres::PgPoolOptions, Executor, Postgres};
use serde::{Deserialize, Serialize};
use futures::{TryFutureExt, TryStreamExt};
use jsonwebtoken::{decode, encode, jwk::JwkSet, Algorithm, DecodingKey, EncodingKey, Header, Validation};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    exp: usize,
}
#[derive(Serialize,Deserialize, Debug)]
struct LogInWithJwt {
    jwt: String,
    familyName: Option<String>,
    givenName: Option<String>,
    email: Option<String>,
}
#[derive(Serialize,Deserialize, Debug)]
struct Event {
    id: i32,
    title: String,
    desc: String,
    public: bool
}
#[derive(Deserialize, Debug)]
struct CreateEventPost {
    event: Event
}
#[derive(Serialize)]
struct DiscoverResponse {
    resp: Vec<Event>
}

async fn create_event(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<Arc<AppState>>,
    axum::Json(payload): axum::Json<CreateEventPost>,
    
) -> StatusCode {
    // insert your application logic here
 let mut execute_query: sqlx::query::Query<Postgres, sqlx::postgres::PgArguments> = sqlx::query(r#"INSERT INTO "public"."events"("title", "desc", "public") VALUES($1, $2, $3) RETURNING "id", "title", "desc", "public";"#).bind(payload.event.title).bind(payload.event.desc).bind(payload.event.public);
    // this will be converted into a JSON response\
    state.con.fetch(execute_query).try_next().await.unwrap();
    // with a status code of `201 Created`
    StatusCode::CREATED
}
async fn signin_apple(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<Arc<AppState>>,
    axum::Json(payload): axum::Json<LogInWithJwt>,
    //first verify with JWT

    // create_user_if needed

) -> StatusCode {
    let jwk_set = reqwest::get("https://appleid.apple.com/auth/keys")
        .await.unwrap()
        .json::<JwkSet>()
        .await.unwrap();
    for jwk in jwk_set.keys {
        if RSA(r) = jwk.algorithm {
            let token = decode::<Claims>(&payload.jwt, &DecodingKey::from_rsa_components(jwk.common., jwk["e"]).unwrap(),&Validation::new(Algorithm::RS256)).unwrap();

        }

    }
    // insert your application logic here
// let mut execute_query: sqlx::query::Query<Postgres, sqlx::postgres::PgArguments> = sqlx::query(r#"INSERT INTO "public"."events"("title", "desc", "public") VALUES($1, $2, $3) RETURNING "id", "title", "desc", "public";"#).bind(payload.event.title).bind(payload.event.desc).bind(payload.event.public);
    // this will be converted into a JSON response\
   // state.con.fetch(execute_query).try_next().await.unwrap();
    // with a status code of `201 Created`
    StatusCode::CREATED
}
#[derive(Clone)]
struct AppState {
    con: sqlx::Pool<Postgres>
}
async fn discover_resp(State(state): State<Arc<AppState>>) -> (StatusCode, axum::Json<DiscoverResponse>)  {

    // insert your application logic here
    let mut fetch_query: sqlx::query::Query<Postgres, sqlx::postgres::PgArguments> = sqlx::query("SELECT * FROM events WHERE public = $1")
    .bind(true);
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
            public: true // if this is wrong then KABOOM!
        })

    }
    let discover_resp = DiscoverResponse {
        resp: events
    };
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

tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");
    // build our application with a single route    
            let con = PgPoolOptions::new().max_connections(5).connect("postgres://postgres:12345678@localhost/postgres").await;
if let Err(nig) = con
     {
        error!("Error while connecting");
        std::process::exit(1);
    }
    let con = con.unwrap();
    info!("DB Connected!");

    let app = Router::new().route("/", get(|| async { "Hello, World!" })).route("/discover_events", get(discover_resp)).route("/signin_apple", post(signin_apple)).route("/create_event", post(create_event)).with_state(AppState {con}.into());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Starting server at 0.0.0.0:3000");
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    info!("Connecting to DB");
   
    axum::serve(listener, app).await.unwrap();
    
}